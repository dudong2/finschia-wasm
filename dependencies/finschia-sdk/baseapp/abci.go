package baseapp

import (
	"crypto/sha256"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"sort"
	"strings"
	"syscall"
	"time"

	"github.com/gogo/protobuf/proto"
	tmproto "github.com/tendermint/tendermint/proto/tendermint/types"
	"google.golang.org/grpc/codes"
	grpcstatus "google.golang.org/grpc/status"

	abci "github.com/tendermint/tendermint/abci/types"

	"github.com/Finschia/finschia-sdk/codec"
	snapshottypes "github.com/Finschia/finschia-sdk/snapshots/types"
	"github.com/Finschia/finschia-sdk/telemetry"
	sdk "github.com/Finschia/finschia-sdk/types"
	sdkerrors "github.com/Finschia/finschia-sdk/types/errors"
	ocabci "github.com/Finschia/ostracon/abci/types"
)

// InitChain implements the ABCI interface. It runs the initialization logic
// directly on the CommitMultiStore.
func (app *BaseApp) InitChain(req abci.RequestInitChain) (res abci.ResponseInitChain) {
	// On a new chain, we consider the init chain block height as 0, even though
	// req.InitialHeight is 1 by default.
	initHeader := tmproto.Header{ChainID: req.ChainId, Time: req.Time}

	// If req.InitialHeight is > 1, then we set the initial version in the
	// stores.
	if req.InitialHeight > 1 {
		app.initialHeight = req.InitialHeight
		initHeader = tmproto.Header{ChainID: req.ChainId, Height: req.InitialHeight, Time: req.Time}
		err := app.cms.SetInitialVersion(req.InitialHeight)
		if err != nil {
			panic(err)
		}
	}

	// initialize the deliver state and check state with a correct header
	app.setDeliverState(initHeader)
	app.setCheckState(initHeader)

	// Store the consensus params in the BaseApp's paramstore. Note, this must be
	// done after the deliver state and context have been set as it's persisted
	// to state.
	if req.ConsensusParams != nil {
		app.StoreConsensusParams(app.deliverState.ctx, req.ConsensusParams)
	}

	if app.initChainer == nil {
		return
	}

	// add block gas meter for any genesis transactions (allow infinite gas)
	app.deliverState.ctx = app.deliverState.ctx.WithBlockGasMeter(sdk.NewInfiniteGasMeter())

	res = app.initChainer(app.deliverState.ctx, req)

	// sanity check
	if len(req.Validators) > 0 {
		if len(req.Validators) != len(res.Validators) {
			panic(
				fmt.Errorf(
					"len(RequestInitChain.Validators) != len(GenesisValidators) (%d != %d)",
					len(req.Validators), len(res.Validators),
				),
			)
		}

		sort.Sort(abci.ValidatorUpdates(req.Validators))
		sort.Sort(abci.ValidatorUpdates(res.Validators))

		for i := range res.Validators {
			if !proto.Equal(&res.Validators[i], &req.Validators[i]) {
				panic(fmt.Errorf("genesisValidators[%d] != req.Validators[%d] ", i, i))
			}
		}
	}

	// In the case of a new chain, AppHash will be the hash of an empty string.
	// During an upgrade, it'll be the hash of the last committed block.
	var appHash []byte
	if !app.LastCommitID().IsZero() {
		appHash = app.LastCommitID().Hash
	} else {
		// $ echo -n '' | sha256sum
		// e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
		emptyHash := sha256.Sum256([]byte{})
		appHash = emptyHash[:]
	}

	// NOTE: We don't commit, but BeginBlock for block `initial_height` starts from this
	// deliverState.
	return abci.ResponseInitChain{
		ConsensusParams: res.ConsensusParams,
		Validators:      res.Validators,
		AppHash:         appHash,
	}
}

// Info implements the ABCI interface.
func (app *BaseApp) Info(req abci.RequestInfo) abci.ResponseInfo {
	lastCommitID := app.cms.LastCommitID()

	return abci.ResponseInfo{
		Data:             app.name,
		Version:          app.version,
		AppVersion:       app.appVersion,
		LastBlockHeight:  lastCommitID.Version,
		LastBlockAppHash: lastCommitID.Hash,
	}
}

// SetOption implements the ABCI interface.
func (app *BaseApp) SetOption(req abci.RequestSetOption) (res abci.ResponseSetOption) {
	return abci.ResponseSetOption{}
}

// BeginBlock implements the ABCI application interface.
func (app *BaseApp) BeginBlock(req ocabci.RequestBeginBlock) (res abci.ResponseBeginBlock) {
	defer telemetry.MeasureSince(time.Now(), "abci", "begin_block")

	if app.cms.TracingEnabled() {
		app.cms.SetTracingContext(sdk.TraceContext(
			map[string]interface{}{"blockHeight": req.Header.Height},
		))
	}

	if err := app.validateHeight(req); err != nil {
		panic(err)
	}

	// set the signed validators for addition to context in deliverTx
	app.voteInfos = req.LastCommitInfo.GetVotes()

	// Initialize the DeliverTx state. If this is the first block, it should
	// already be initialized in InitChain. Otherwise app.deliverState will be
	// nil, since it is reset on Commit.
	if app.deliverState == nil {
		app.setDeliverState(req.Header)
	} else {
		// In the first block, app.deliverState.ctx will already be initialized
		// by InitChain. Context is now updated with Header information.
		app.deliverState.ctx = app.deliverState.ctx.
			WithBlockHeader(req.Header).
			WithBlockHeight(req.Header.Height)
	}

	// add block gas meter
	var gasMeter sdk.GasMeter
	if maxGas := app.getMaximumBlockGas(app.deliverState.ctx); maxGas > 0 {
		gasMeter = sdk.NewGasMeter(maxGas)
	} else {
		gasMeter = sdk.NewInfiniteGasMeter()
	}

	// NOTE: header hash is not set in NewContext, so we manually set it here

	app.deliverState.ctx = app.deliverState.ctx.
		WithVoteInfos(app.voteInfos).
		WithBlockGasMeter(gasMeter).
		WithHeaderHash(req.Hash).
		WithConsensusParams(app.GetConsensusParams(app.deliverState.ctx))

	// we also set block gas meter to checkState in case the application needs to
	// verify gas consumption during (Re)CheckTx
	if app.checkState != nil {
		app.checkState.ctx = app.checkState.ctx.
			WithBlockGasMeter(gasMeter).
			WithHeaderHash(req.Hash)
	}

	if app.beginBlocker != nil {
		res = app.beginBlocker(app.deliverState.ctx, req)
		res.Events = sdk.MarkEventsToIndex(res.Events, app.indexEvents)
	}
	// set the signed validators for addition to context in deliverTx
	app.voteInfos = req.LastCommitInfo.GetVotes()

	// call the hooks with the BeginBlock messages
	for _, streamingListener := range app.abciListeners {
		if err := streamingListener.ListenBeginBlock(app.deliverState.ctx, req, res); err != nil {
			app.logger.Error("BeginBlock listening hook failed", "height", req.Header.Height, "err", err)
		}
	}

	return res
}

// EndBlock implements the ABCI interface.
func (app *BaseApp) EndBlock(req abci.RequestEndBlock) (res abci.ResponseEndBlock) {
	defer telemetry.MeasureSince(time.Now(), "abci", "end_block")

	if app.deliverState.ms.TracingEnabled() {
		app.deliverState.ms = app.deliverState.ms.SetTracingContext(nil).(sdk.CacheMultiStore)
	}

	if app.endBlocker != nil {
		res = app.endBlocker(app.deliverState.ctx, req)
		res.Events = sdk.MarkEventsToIndex(res.Events, app.indexEvents)
	}

	if cp := app.GetConsensusParams(app.deliverState.ctx); cp != nil {
		res.ConsensusParamUpdates = cp
	}

	// call the streaming service hooks with the EndBlock messages
	for _, streamingListener := range app.abciListeners {
		if err := streamingListener.ListenEndBlock(app.deliverState.ctx, req, res); err != nil {
			app.logger.Error("EndBlock listening hook failed", "height", req.Height, "err", err)
		}
	}

	return res
}

// CheckTx implements the ABCI interface and executes a tx in CheckTx mode. In
// CheckTx mode, messages are not executed. This means messages are only validated
// and only the AnteHandler is executed. State is persisted to the BaseApp's
// internal CheckTx state if the AnteHandler passes. Otherwise, the ResponseCheckTx
// will contain releveant error information. Regardless of tx execution outcome,
// the ResponseCheckTx will contain relevant gas execution context.
func (app *BaseApp) CheckTxSync(req abci.RequestCheckTx) ocabci.ResponseCheckTx {
	defer telemetry.MeasureSince(time.Now(), "abci", "check_tx")

	if req.Type != abci.CheckTxType_New && req.Type != abci.CheckTxType_Recheck {
		panic(fmt.Sprintf("unknown RequestCheckTx type: %s", req.Type))
	}

	tx, err := app.preCheckTx(req.Tx)
	if err != nil {
		return sdkerrors.ResponseCheckTx(err, 0, 0, app.trace)
	}

	waits, signals := app.checkAccountWGs.Register(tx)

	app.checkAccountWGs.Wait(waits)
	defer app.checkAccountWGs.Done(signals)

	gInfo, err := app.checkTx(req.Tx, tx, req.Type == abci.CheckTxType_Recheck)
	if err != nil {
		return sdkerrors.ResponseCheckTx(err, gInfo.GasWanted, gInfo.GasUsed, app.trace)
		// return sdkerrors.ResponseCheckTxWithEvents(err, gInfo.GasWanted, gInfo.GasUsed, anteEvents, app.trace) // TODO(dudong2): need to fix to use ResponseCheckTxWithEvents
	}

	return ocabci.ResponseCheckTx{
		GasWanted: int64(gInfo.GasWanted), // TODO: Should type accept unsigned ints?
		GasUsed:   int64(gInfo.GasUsed),   // TODO: Should type accept unsigned ints?
	}
}

func (app *BaseApp) CheckTxAsync(req abci.RequestCheckTx, callback ocabci.CheckTxCallback) {
	if req.Type != abci.CheckTxType_New && req.Type != abci.CheckTxType_Recheck {
		panic(fmt.Sprintf("unknown RequestCheckTx type: %s", req.Type))
	}

	reqCheckTx := &RequestCheckTxAsync{
		txBytes:  req.Tx,
		recheck:  req.Type == abci.CheckTxType_Recheck,
		callback: callback,
		prepare:  waitGroup1(),
	}
	app.chCheckTx <- reqCheckTx

	go app.prepareCheckTx(reqCheckTx)
}

// BeginRecheckTx implements the ABCI interface and set the check state based on the given header
func (app *BaseApp) BeginRecheckTx(req ocabci.RequestBeginRecheckTx) ocabci.ResponseBeginRecheckTx {
	// NOTE: This is safe because Ostracon holds a lock on the mempool for Rechecking.
	app.setCheckState(req.Header)
	return ocabci.ResponseBeginRecheckTx{Code: abci.CodeTypeOK}
}

// EndRecheckTx implements the ABCI interface.
func (app *BaseApp) EndRecheckTx(req ocabci.RequestEndRecheckTx) ocabci.ResponseEndRecheckTx {
	return ocabci.ResponseEndRecheckTx{Code: abci.CodeTypeOK}
}

// DeliverTx implements the ABCI interface and executes a tx in DeliverTx mode.
// State only gets persisted if all messages are valid and get executed successfully.
// Otherwise, the ResponseDeliverTx will contain releveant error information.
// Regardless of tx execution outcome, the ResponseDeliverTx will contain relevant
// gas execution context.
func (app *BaseApp) DeliverTx(req abci.RequestDeliverTx) (res abci.ResponseDeliverTx) {
	defer telemetry.MeasureSince(time.Now(), "abci", "deliver_tx")

	defer func() {
		for _, streamingListener := range app.abciListeners {
			if err := streamingListener.ListenDeliverTx(app.deliverState.ctx, req, res); err != nil {
				app.logger.Error("DeliverTx listening hook failed", "err", err)
			}
		}
	}()

	gInfo := sdk.GasInfo{}
	resultStr := "successful"

	defer func() {
		telemetry.IncrCounter(1, "tx", "count")
		telemetry.IncrCounter(1, "tx", resultStr)
		telemetry.SetGauge(float32(gInfo.GasUsed), "tx", "gas", "used")
		telemetry.SetGauge(float32(gInfo.GasWanted), "tx", "gas", "wanted")
	}()

	tx, err := app.txDecoder(req.Tx)
	if err != nil {
		return sdkerrors.ResponseDeliverTx(err, 0, 0, app.trace)
	}

	gInfo, result, anteEvents, err := app.runTx(req.Tx, tx, false)
	if err != nil {
		resultStr = "failed"
		return sdkerrors.ResponseDeliverTxWithEvents(err, gInfo.GasWanted, gInfo.GasUsed, sdk.MarkEventsToIndex(anteEvents, app.indexEvents), app.trace)
	}

	return abci.ResponseDeliverTx{
		GasWanted: int64(gInfo.GasWanted), // TODO: Should type accept unsigned ints?
		GasUsed:   int64(gInfo.GasUsed),   // TODO: Should type accept unsigned ints?
		Log:       result.Log,
		Data:      result.Data,
		Events:    sdk.MarkEventsToIndex(result.Events, app.indexEvents),
	}
}

// Commit implements the ABCI interface. It will commit all state that exists in
// the deliver state's multi-store and includes the resulting commit ID in the
// returned abci.ResponseCommit. Commit will reset the deliver state.
// Also, if a non-zero halt height is defined in config, Commit will execute
// a deferred function call to check against that height and gracefully halt if
// it matches the latest committed height.
func (app *BaseApp) Commit() (res abci.ResponseCommit) {
	defer telemetry.MeasureSince(time.Now(), "abci", "commit")

	header := app.deliverState.ctx.BlockHeader()
	retainHeight := app.GetBlockRetentionHeight(header.Height)

	// Write the DeliverTx state into branched storage and commit the MultiStore.
	// The write to the DeliverTx state writes all state transitions to the root
	// MultiStore (app.cms) so when Commit() is called is persists those values.
	app.deliverState.ms.Write()
	commitID := app.cms.Commit()
	app.logger.Info("commit synced", "commit", fmt.Sprintf("%X", commitID))

	// empty/reset the deliver state
	app.deliverState = nil

	var halt bool

	switch {
	case app.haltHeight > 0 && uint64(header.Height) >= app.haltHeight:
		halt = true

	case app.haltTime > 0 && header.Time.Unix() >= int64(app.haltTime):
		halt = true
	}

	if halt {
		// Halt the binary and allow Tendermint to receive the ResponseCommit
		// response with the commit ID hash. This will allow the node to successfully
		// restart and process blocks assuming the halt configuration has been
		// reset or moved to a more distant value.
		app.halt()
	}

	if app.snapshotInterval > 0 && uint64(header.Height)%app.snapshotInterval == 0 {
		go app.snapshot(header.Height)
	}

	return abci.ResponseCommit{
		Data:         commitID.Hash,
		RetainHeight: retainHeight,
	}
}

// halt attempts to gracefully shutdown the node via SIGINT and SIGTERM falling
// back on os.Exit if both fail.
func (app *BaseApp) halt() {
	app.logger.Info("halting node per configuration", "height", app.haltHeight, "time", app.haltTime)

	p, err := os.FindProcess(os.Getpid())
	if err == nil {
		// attempt cascading signals in case SIGINT fails (os dependent)
		sigIntErr := p.Signal(syscall.SIGINT)
		sigTermErr := p.Signal(syscall.SIGTERM)

		if sigIntErr == nil || sigTermErr == nil {
			return
		}
	}

	// Resort to exiting immediately if the process could not be found or killed
	// via SIGINT/SIGTERM signals.
	app.logger.Info("failed to send SIGINT/SIGTERM; exiting...")
	os.Exit(0)
}

// snapshot takes a snapshot of the current state and prunes any old snapshottypes.
func (app *BaseApp) snapshot(height int64) {
	if app.snapshotManager == nil {
		app.logger.Info("snapshot manager not configured")
		return
	}

	app.logger.Info("creating state snapshot", "height", height)

	snapshot, err := app.snapshotManager.Create(uint64(height))
	if err != nil {
		app.logger.Error("failed to create state snapshot", "height", height, "err", err)
		return
	}

	app.logger.Info("completed state snapshot", "height", height, "format", snapshot.Format)

	if app.snapshotKeepRecent > 0 {
		app.logger.Debug("pruning state snapshots")

		pruned, err := app.snapshotManager.Prune(app.snapshotKeepRecent)
		if err != nil {
			app.logger.Error("Failed to prune state snapshots", "err", err)
			return
		}

		app.logger.Debug("pruned state snapshots", "pruned", pruned)
	}
}

// Query implements the ABCI interface. It delegates to CommitMultiStore if it
// implements Queryable.
func (app *BaseApp) Query(req abci.RequestQuery) (res abci.ResponseQuery) {
	defer telemetry.MeasureSince(time.Now(), "abci", "query")

	// Add panic recovery for all queries.
	// ref: https://github.com/cosmos/cosmos-sdk/pull/8039
	defer func() {
		if r := recover(); r != nil {
			res = sdkerrors.QueryResultWithDebug(sdkerrors.Wrapf(sdkerrors.ErrPanic, "%v", r), app.trace)
		}
	}()

	// when a client did not provide a query height, manually inject the latest
	if req.Height == 0 {
		req.Height = app.LastBlockHeight()
	}

	// handle gRPC routes first rather than calling splitPath because '/' characters
	// are used as part of gRPC paths
	if grpcHandler := app.grpcQueryRouter.Route(req.Path); grpcHandler != nil {
		return app.handleQueryGRPC(grpcHandler, req)
	}

	path := splitPath(req.Path)
	if len(path) == 0 {
		return sdkerrors.QueryResultWithDebug(sdkerrors.Wrap(sdkerrors.ErrUnknownRequest, "no query path provided"), app.trace)
	}

	switch path[0] {
	// "/app" prefix for special application queries
	case "app":
		return handleQueryApp(app, path, req)

	case "store":
		return handleQueryStore(app, path, req)

	case "p2p":
		return handleQueryP2P(app, path)

	case "custom":
		return handleQueryCustom(app, path, req)
	}

	return sdkerrors.QueryResultWithDebug(sdkerrors.Wrap(sdkerrors.ErrUnknownRequest, "unknown query path"), app.trace)
}

// ListSnapshots implements the ABCI interface. It delegates to app.snapshotManager if set.
func (app *BaseApp) ListSnapshots(req abci.RequestListSnapshots) abci.ResponseListSnapshots {
	resp := abci.ResponseListSnapshots{Snapshots: []*abci.Snapshot{}}
	if app.snapshotManager == nil {
		return resp
	}

	snapshots, err := app.snapshotManager.List()
	if err != nil {
		app.logger.Error("failed to list snapshots", "err", err)
		return resp
	}

	for _, snapshot := range snapshots {
		abciSnapshot, err := snapshot.ToABCI()
		if err != nil {
			app.logger.Error("failed to list snapshots", "err", err)
			return resp
		}
		resp.Snapshots = append(resp.Snapshots, &abciSnapshot)
	}

	return resp
}

// LoadSnapshotChunk implements the ABCI interface. It delegates to app.snapshotManager if set.
func (app *BaseApp) LoadSnapshotChunk(req abci.RequestLoadSnapshotChunk) abci.ResponseLoadSnapshotChunk {
	if app.snapshotManager == nil {
		return abci.ResponseLoadSnapshotChunk{}
	}
	chunk, err := app.snapshotManager.LoadChunk(req.Height, req.Format, req.Chunk)
	if err != nil {
		app.logger.Error(
			"failed to load snapshot chunk",
			"height", req.Height,
			"format", req.Format,
			"chunk", req.Chunk,
			"err", err,
		)
		return abci.ResponseLoadSnapshotChunk{}
	}
	return abci.ResponseLoadSnapshotChunk{Chunk: chunk}
}

// OfferSnapshot implements the ABCI interface. It delegates to app.snapshotManager if set.
func (app *BaseApp) OfferSnapshot(req abci.RequestOfferSnapshot) abci.ResponseOfferSnapshot {
	if app.snapshotManager == nil {
		app.logger.Error("snapshot manager not configured")
		return abci.ResponseOfferSnapshot{Result: abci.ResponseOfferSnapshot_ABORT}
	}

	if req.Snapshot == nil {
		app.logger.Error("received nil snapshot")
		return abci.ResponseOfferSnapshot{Result: abci.ResponseOfferSnapshot_REJECT}
	}

	snapshot, err := snapshottypes.SnapshotFromABCI(req.Snapshot)
	if err != nil {
		app.logger.Error("failed to decode snapshot metadata", "err", err)
		return abci.ResponseOfferSnapshot{Result: abci.ResponseOfferSnapshot_REJECT}
	}

	err = app.snapshotManager.Restore(snapshot)
	switch {
	case err == nil:
		return abci.ResponseOfferSnapshot{Result: abci.ResponseOfferSnapshot_ACCEPT}

	case errors.Is(err, snapshottypes.ErrUnknownFormat):
		return abci.ResponseOfferSnapshot{Result: abci.ResponseOfferSnapshot_REJECT_FORMAT}

	case errors.Is(err, snapshottypes.ErrInvalidMetadata):
		app.logger.Error(
			"rejecting invalid snapshot",
			"height", req.Snapshot.Height,
			"format", req.Snapshot.Format,
			"err", err,
		)
		return abci.ResponseOfferSnapshot{Result: abci.ResponseOfferSnapshot_REJECT}

	default:
		app.logger.Error(
			"failed to restore snapshot",
			"height", req.Snapshot.Height,
			"format", req.Snapshot.Format,
			"err", err,
		)

		// We currently don't support resetting the IAVL stores and retrying a different snapshot,
		// so we ask Tendermint to abort all snapshot restoration.
		return abci.ResponseOfferSnapshot{Result: abci.ResponseOfferSnapshot_ABORT}
	}
}

// ApplySnapshotChunk implements the ABCI interface. It delegates to app.snapshotManager if set.
func (app *BaseApp) ApplySnapshotChunk(req abci.RequestApplySnapshotChunk) abci.ResponseApplySnapshotChunk {
	if app.snapshotManager == nil {
		app.logger.Error("snapshot manager not configured")
		return abci.ResponseApplySnapshotChunk{Result: abci.ResponseApplySnapshotChunk_ABORT}
	}

	_, err := app.snapshotManager.RestoreChunk(req.Chunk)
	switch {
	case err == nil:
		return abci.ResponseApplySnapshotChunk{Result: abci.ResponseApplySnapshotChunk_ACCEPT}

	case errors.Is(err, snapshottypes.ErrChunkHashMismatch):
		app.logger.Error(
			"chunk checksum mismatch; rejecting sender and requesting refetch",
			"chunk", req.Index,
			"sender", req.Sender,
			"err", err,
		)
		return abci.ResponseApplySnapshotChunk{
			Result:        abci.ResponseApplySnapshotChunk_RETRY,
			RefetchChunks: []uint32{req.Index},
			RejectSenders: []string{req.Sender},
		}

	default:
		app.logger.Error("failed to restore snapshot", "err", err)
		return abci.ResponseApplySnapshotChunk{Result: abci.ResponseApplySnapshotChunk_ABORT}
	}
}

func (app *BaseApp) handleQueryGRPC(handler GRPCQueryHandler, req abci.RequestQuery) abci.ResponseQuery {
	ctx, err := app.createQueryContext(req.Height, req.Prove)
	if err != nil {
		return sdkerrors.QueryResultWithDebug(err, app.trace)
	}

	res, err := handler(ctx, req)
	if err != nil {
		res = sdkerrors.QueryResultWithDebug(gRPCErrorToSDKError(err), app.trace)
		res.Height = req.Height
		return res
	}

	return res
}

func gRPCErrorToSDKError(err error) error {
	status, ok := grpcstatus.FromError(err)
	if !ok {
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, err.Error())
	}

	switch status.Code() {
	case codes.NotFound:
		return sdkerrors.Wrap(sdkerrors.ErrKeyNotFound, err.Error())
	case codes.InvalidArgument:
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, err.Error())
	case codes.FailedPrecondition:
		return sdkerrors.Wrap(sdkerrors.ErrInvalidRequest, err.Error())
	case codes.Unauthenticated:
		return sdkerrors.Wrap(sdkerrors.ErrUnauthorized, err.Error())
	default:
		return sdkerrors.Wrap(sdkerrors.ErrUnknownRequest, err.Error())
	}
}

func checkNegativeHeight(height int64) error {
	if height < 0 {
		// Reject invalid heights.
		return sdkerrors.Wrap(
			sdkerrors.ErrInvalidRequest,
			"cannot query with height < 0; please provide a valid height",
		)
	}
	return nil
}

// createQueryContext creates a new sdk.Context for a query, taking as args
// the block height and whether the query needs a proof or not.
func (app *BaseApp) createQueryContext(height int64, prove bool) (sdk.Context, error) {
	if err := checkNegativeHeight(height); err != nil {
		return sdk.Context{}, err
	}

	lastBlockHeight := app.LastBlockHeight()
	if height > lastBlockHeight {
		return sdk.Context{},
			sdkerrors.Wrap(
				sdkerrors.ErrInvalidHeight,
				"cannot query with height in the future; please provide a valid height",
			)
	}

	// when a client did not provide a query height, manually inject the latest
	if height == 0 {
		height = lastBlockHeight
	}

	if height <= 1 && prove {
		return sdk.Context{},
			sdkerrors.Wrap(
				sdkerrors.ErrInvalidRequest,
				"cannot query with proof when height <= 1; please provide a valid height",
			)
	}

	cacheMS, err := app.cms.CacheMultiStoreWithVersion(height)
	if err != nil {
		return sdk.Context{},
			sdkerrors.Wrapf(
				sdkerrors.ErrInvalidRequest,
				"failed to load state at height %d; %s (latest height: %d)", height, err, lastBlockHeight,
			)
	}

	// branch the commit-multistore for safety
	app.checkStateMtx.RLock()
	ctx := sdk.NewContext(
		cacheMS, app.checkState.ctx.BlockHeader(), true, app.logger,
	).WithMinGasPrices(app.minGasPrices).WithBlockHeight(height)
	app.checkStateMtx.RUnlock()

	return ctx, nil
}

// GetBlockRetentionHeight returns the height for which all blocks below this height
// are pruned from Tendermint. Given a commitment height and a non-zero local
// minRetainBlocks configuration, the retentionHeight is the smallest height that
// satisfies:
//
// - Unbonding (safety threshold) time: The block interval in which validators
// can be economically punished for misbehavior. Blocks in this interval must be
// auditable e.g. by the light client.
//
// - Logical store snapshot interval: The block interval at which the underlying
// logical store database is persisted to disk, e.g. every 10000 heights. Blocks
// since the last IAVL snapshot must be available for replay on application restart.
//
// - State sync snapshots: Blocks since the oldest available snapshot must be
// available for state sync nodes to catch up (oldest because a node may be
// restoring an old snapshot while a new snapshot was taken).
//
// - Local (minRetainBlocks) config: Archive nodes may want to retain more or
// all blocks, e.g. via a local config option min-retain-blocks. There may also
// be a need to vary retention for other nodes, e.g. sentry nodes which do not
// need historical blocks.
func (app *BaseApp) GetBlockRetentionHeight(commitHeight int64) int64 {
	// pruning is disabled if minRetainBlocks is zero
	if app.minRetainBlocks == 0 {
		return 0
	}

	minNonZero := func(x, y int64) int64 {
		switch {
		case x == 0:
			return y
		case y == 0:
			return x
		case x < y:
			return x
		default:
			return y
		}
	}

	// Define retentionHeight as the minimum value that satisfies all non-zero
	// constraints. All blocks below (commitHeight-retentionHeight) are pruned
	// from Tendermint.
	var retentionHeight int64

	// Define the number of blocks needed to protect against misbehaving validators
	// which allows light clients to operate safely. Note, we piggy back of the
	// evidence parameters instead of computing an estimated nubmer of blocks based
	// on the unbonding period and block commitment time as the two should be
	// equivalent.
	cp := app.GetConsensusParams(app.deliverState.ctx)
	if cp != nil && cp.Evidence != nil && cp.Evidence.MaxAgeNumBlocks > 0 {
		retentionHeight = commitHeight - cp.Evidence.MaxAgeNumBlocks
	}

	// Define the state pruning offset, i.e. the block offset at which the
	// underlying logical database is persisted to disk.
	statePruningOffset := int64(app.cms.GetPruning().KeepEvery)
	if statePruningOffset > 0 {
		if commitHeight > statePruningOffset {
			v := commitHeight - (commitHeight % statePruningOffset)
			retentionHeight = minNonZero(retentionHeight, v)
		} else {
			// Hitting this case means we have persisting enabled but have yet to reach
			// a height in which we persist state, so we return zero regardless of other
			// conditions. Otherwise, we could end up pruning blocks without having
			// any state committed to disk.
			return 0
		}
	}

	if app.snapshotInterval > 0 && app.snapshotKeepRecent > 0 {
		v := commitHeight - int64((app.snapshotInterval * uint64(app.snapshotKeepRecent)))
		retentionHeight = minNonZero(retentionHeight, v)
	}

	v := commitHeight - int64(app.minRetainBlocks)
	retentionHeight = minNonZero(retentionHeight, v)

	if retentionHeight <= 0 {
		// prune nothing in the case of a non-positive height
		return 0
	}

	return retentionHeight
}

func handleQueryApp(app *BaseApp, path []string, req abci.RequestQuery) abci.ResponseQuery {
	if len(path) >= 2 {
		switch path[1] {
		case "simulate":
			txBytes := req.Data

			gInfo, res, err := app.Simulate(txBytes)
			if err != nil {
				return sdkerrors.QueryResultWithDebug(sdkerrors.Wrap(err, "failed to simulate tx"), app.trace)
			}

			simRes := &sdk.SimulationResponse{
				GasInfo: gInfo,
				Result:  res,
			}

			bz, err := codec.ProtoMarshalJSON(simRes, app.interfaceRegistry)
			if err != nil {
				return sdkerrors.QueryResultWithDebug(sdkerrors.Wrap(err, "failed to JSON encode simulation response"), app.trace)
			}

			return abci.ResponseQuery{
				Codespace: sdkerrors.RootCodespace,
				Height:    req.Height,
				Value:     bz,
			}

		case "version":
			return abci.ResponseQuery{
				Codespace: sdkerrors.RootCodespace,
				Height:    req.Height,
				Value:     []byte(app.version),
			}

		case "snapshots":
			var responseValue []byte

			response := app.ListSnapshots(abci.RequestListSnapshots{})

			responseValue, err := json.Marshal(response)
			if err != nil {
				sdkerrors.QueryResult(sdkerrors.Wrap(err, fmt.Sprintf("failed to marshal list snapshots response %v", response)))
			}

			return abci.ResponseQuery{
				Codespace: sdkerrors.RootCodespace,
				Height:    req.Height,
				Value:     responseValue,
			}

		default:
			return sdkerrors.QueryResultWithDebug(sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "unknown query: %s", path), app.trace)
		}
	}

	return sdkerrors.QueryResultWithDebug(
		sdkerrors.Wrap(
			sdkerrors.ErrUnknownRequest,
			"expected second parameter to be either 'simulate' or 'version', neither was present",
		), app.trace)
}

func handleQueryStore(app *BaseApp, path []string, req abci.RequestQuery) abci.ResponseQuery {
	// "/store" prefix for store queries
	queryable, ok := app.cms.(sdk.Queryable)
	if !ok {
		return sdkerrors.QueryResultWithDebug(sdkerrors.Wrap(sdkerrors.ErrUnknownRequest, "multistore doesn't support queries"), app.trace)
	}

	req.Path = "/" + strings.Join(path[1:], "/")

	if req.Height <= 1 && req.Prove {
		return sdkerrors.QueryResultWithDebug(
			sdkerrors.Wrap(
				sdkerrors.ErrInvalidRequest,
				"cannot query with proof when height <= 1; please provide a valid height",
			), app.trace)
	}

	resp := queryable.Query(req)
	resp.Height = req.Height

	return resp
}

func handleQueryCustom(app *BaseApp, path []string, req abci.RequestQuery) abci.ResponseQuery {
	// path[0] should be "custom" because "/custom" prefix is required for keeper
	// queries.
	//
	// The QueryRouter routes using path[1]. For example, in the path
	// "custom/gov/proposal", QueryRouter routes using "gov".
	if len(path) < 2 || path[1] == "" {
		return sdkerrors.QueryResultWithDebug(sdkerrors.Wrap(sdkerrors.ErrUnknownRequest, "no route for custom query specified"), app.trace)
	}

	querier := app.queryRouter.Route(path[1])
	if querier == nil {
		return sdkerrors.QueryResultWithDebug(sdkerrors.Wrapf(sdkerrors.ErrUnknownRequest, "no custom querier found for route %s", path[1]), app.trace)
	}

	ctx, err := app.createQueryContext(req.Height, req.Prove)
	if err != nil {
		return sdkerrors.QueryResultWithDebug(err, app.trace)
	}

	// Passes the rest of the path as an argument to the querier.
	//
	// For example, in the path "custom/gov/proposal/test", the gov querier gets
	// []string{"proposal", "test"} as the path.
	resBytes, err := querier(ctx, path[2:], req)
	if err != nil {
		res := sdkerrors.QueryResultWithDebug(err, app.trace)
		res.Height = req.Height
		return res
	}

	return abci.ResponseQuery{
		Height: req.Height,
		Value:  resBytes,
	}
}

// splitPath splits a string path using the delimiter '/'.
//
// e.g. "this/is/funny" becomes []string{"this", "is", "funny"}
func splitPath(requestPath string) (path []string) {
	path = strings.Split(requestPath, "/")

	// first element is empty string
	if len(path) > 0 && path[0] == "" {
		path = path[1:]
	}

	return path
}

// createQueryContext creates a new sdk.Context for a query, taking as args
// the block height and whether the query needs a proof or not.
func (app *BaseApp) createQueryContextWithCheckState() sdk.Context {

	cacheMS := app.checkState.CacheMultiStore()

	// branch the commit-multistore for safety
	app.checkStateMtx.RLock()
	ctx := sdk.NewContext(
		cacheMS, app.checkState.ctx.BlockHeader(), true, app.logger,
	).WithMinGasPrices(app.minGasPrices).WithBlockHeight(app.LastBlockHeight())
	app.checkStateMtx.RUnlock()

	return ctx
}
package keyring

import (
	"github.com/Finschia/finschia-sdk/codec"
	"github.com/Finschia/finschia-sdk/codec/legacy"
	"github.com/Finschia/finschia-sdk/crypto/hd"
)

func init() {
	RegisterLegacyAminoCodec(legacy.Cdc)
}

// RegisterLegacyAminoCodec registers concrete types and interfaces on the given codec.
func RegisterLegacyAminoCodec(cdc *codec.LegacyAmino) {
	cdc.RegisterInterface((*Info)(nil), nil)
	cdc.RegisterConcrete(hd.BIP44Params{}, "crypto/keys/hd/BIP44Params", nil)
	cdc.RegisterConcrete(localInfo{}, "crypto/keys/localInfo", nil)
	cdc.RegisterConcrete(ledgerInfo{}, "crypto/keys/ledgerInfo", nil)
	cdc.RegisterConcrete(offlineInfo{}, "crypto/keys/offlineInfo", nil)
	cdc.RegisterConcrete(multiInfo{}, "crypto/keys/multiInfo", nil)
}
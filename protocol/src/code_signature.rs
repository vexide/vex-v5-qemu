use bincode::{
    de::{self, BorrowDecoder, Decoder},
    enc::{self, Encoder},
    error::{DecodeError, EncodeError},
    BorrowDecode, Decode, Encode,
};
use bitflags::bitflags;
use vex_sdk::vcodesig;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Program Code Signature
///
/// The first 16 bytes of a VEX user code binary contain a user code signature,
/// containing some basic metadata and startup flags about the program. This
/// signature must be at the start of the binary for booting to occur.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CodeSignature {
    pub magic: u32,
    pub program_type: ProgramType,
    pub owner: ProgramOwner,
    pub flags: ProgramFlags,
}

// TODO: This impl should probably be TryFrom and not have unreachables
impl From<vcodesig> for CodeSignature {
    fn from(vcodesig: vcodesig) -> Self {
        Self {
            magic: vcodesig.magic,
            program_type: ProgramType::User,
            owner: match vcodesig.owner {
                1 => ProgramOwner::Vex,
                2 => ProgramOwner::Partner,
                _ => ProgramOwner::System,
            },
            flags: ProgramFlags::from_bits_retain(vcodesig.options),
        }
    }
}

/// Identifies the type of binary to VEXos.
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProgramType {
    /// User program binary.
    User = 0,
}

/// The owner (originator) of the user program
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProgramOwner {
    /// Program is a system binary.
    System = 0,

    /// Program originated from VEX.
    Vex = 1,

    /// Program originated from a partner developer.
    Partner = 2,
}

bitflags! {
    /// Program Flags
    ///
    /// These bitflags are part of the [`CodeSignature`] that determine some small
    /// aspects of program behavior when running under VEXos. This struct contains
    /// the flags with publicly documented behavior.
    #[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct ProgramFlags: u32 {
        /// Default graphics colors will be inverted.
        const INVERT_DEFAULT_GRAPHICS = 1 << 0;

        /// VEXos scheduler simple tasks will be killed when the program requests exit.
        const KILL_TASKS_ON_EXIT = 1 << 1;

        /// Default graphics colors will invert based on the selected system theme.
        const THEMED_DEFAULT_GRAPHICS = 1 << 2;
    }
}

impl enc::Encode for ProgramFlags {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Encode::encode(&self.bits(), encoder)?;
        Ok(())
    }
}

impl de::Decode for ProgramFlags {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        Ok(Self::from_bits_retain(Decode::decode(decoder)?))
    }
}

impl<'de> bincode::BorrowDecode<'de> for ProgramFlags {
    fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
        Ok(Self::from_bits_retain(BorrowDecode::borrow_decode(
            decoder,
        )?))
    }
}

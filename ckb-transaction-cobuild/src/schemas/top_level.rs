// Generated by Molecule 0.7.5

use super::basic::*;
use molecule::prelude::*;
#[derive(Clone)]
pub struct WitnessLayout(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for WitnessLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for WitnessLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for WitnessLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}(", Self::NAME)?;
        self.to_enum().display_inner(f)?;
        write!(f, ")")
    }
}
impl ::core::default::Default for WitnessLayout {
    fn default() -> Self {
        let v = molecule::bytes::Bytes::from_static(&Self::DEFAULT_VALUE);
        WitnessLayout::new_unchecked(v)
    }
}
impl WitnessLayout {
    const DEFAULT_VALUE: [u8; 32] = [
        1, 0, 0, 255, 28, 0, 0, 0, 12, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 8, 0, 0, 0,
        4, 0, 0, 0,
    ];
    pub const ITEMS_COUNT: usize = 4;
    pub fn item_id(&self) -> molecule::Number {
        molecule::unpack_number(self.as_slice())
    }
    pub fn to_enum(&self) -> WitnessLayoutUnion {
        let inner = self.0.slice(molecule::NUMBER_SIZE..);
        match self.item_id() {
            4278190081 => SighashAll::new_unchecked(inner).into(),
            4278190082 => SighashAllOnly::new_unchecked(inner).into(),
            4278190083 => Otx::new_unchecked(inner).into(),
            4278190084 => OtxStart::new_unchecked(inner).into(),
            _ => panic!("{}: invalid data", Self::NAME),
        }
    }
    pub fn as_reader<'r>(&'r self) -> WitnessLayoutReader<'r> {
        WitnessLayoutReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for WitnessLayout {
    type Builder = WitnessLayoutBuilder;
    const NAME: &'static str = "WitnessLayout";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        WitnessLayout(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        WitnessLayoutReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        WitnessLayoutReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().set(self.to_enum())
    }
}
#[derive(Clone, Copy)]
pub struct WitnessLayoutReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for WitnessLayoutReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for WitnessLayoutReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for WitnessLayoutReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}(", Self::NAME)?;
        self.to_enum().display_inner(f)?;
        write!(f, ")")
    }
}
impl<'r> WitnessLayoutReader<'r> {
    pub const ITEMS_COUNT: usize = 4;
    pub fn item_id(&self) -> molecule::Number {
        molecule::unpack_number(self.as_slice())
    }
    pub fn to_enum(&self) -> WitnessLayoutUnionReader<'r> {
        let inner = &self.as_slice()[molecule::NUMBER_SIZE..];
        match self.item_id() {
            4278190081 => SighashAllReader::new_unchecked(inner).into(),
            4278190082 => SighashAllOnlyReader::new_unchecked(inner).into(),
            4278190083 => OtxReader::new_unchecked(inner).into(),
            4278190084 => OtxStartReader::new_unchecked(inner).into(),
            _ => panic!("{}: invalid data", Self::NAME),
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for WitnessLayoutReader<'r> {
    type Entity = WitnessLayout;
    const NAME: &'static str = "WitnessLayoutReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        WitnessLayoutReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let item_id = molecule::unpack_number(slice);
        let inner_slice = &slice[molecule::NUMBER_SIZE..];
        match item_id {
            4278190081 => SighashAllReader::verify(inner_slice, compatible),
            4278190082 => SighashAllOnlyReader::verify(inner_slice, compatible),
            4278190083 => OtxReader::verify(inner_slice, compatible),
            4278190084 => OtxStartReader::verify(inner_slice, compatible),
            _ => ve!(Self, UnknownItem, Self::ITEMS_COUNT, item_id),
        }?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct WitnessLayoutBuilder(pub(crate) WitnessLayoutUnion);
impl WitnessLayoutBuilder {
    pub const ITEMS_COUNT: usize = 4;
    pub fn set<I>(mut self, v: I) -> Self
    where
        I: ::core::convert::Into<WitnessLayoutUnion>,
    {
        self.0 = v.into();
        self
    }
}
impl molecule::prelude::Builder for WitnessLayoutBuilder {
    type Entity = WitnessLayout;
    const NAME: &'static str = "WitnessLayoutBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE + self.0.as_slice().len()
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        writer.write_all(&molecule::pack_number(self.0.item_id()))?;
        writer.write_all(self.0.as_slice())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        WitnessLayout::new_unchecked(inner.into())
    }
}
#[derive(Debug, Clone)]
pub enum WitnessLayoutUnion {
    SighashAll(SighashAll),
    SighashAllOnly(SighashAllOnly),
    Otx(Otx),
    OtxStart(OtxStart),
}
#[derive(Debug, Clone, Copy)]
pub enum WitnessLayoutUnionReader<'r> {
    SighashAll(SighashAllReader<'r>),
    SighashAllOnly(SighashAllOnlyReader<'r>),
    Otx(OtxReader<'r>),
    OtxStart(OtxStartReader<'r>),
}
impl ::core::default::Default for WitnessLayoutUnion {
    fn default() -> Self {
        WitnessLayoutUnion::SighashAll(::core::default::Default::default())
    }
}
impl ::core::fmt::Display for WitnessLayoutUnion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            WitnessLayoutUnion::SighashAll(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, SighashAll::NAME, item)
            }
            WitnessLayoutUnion::SighashAllOnly(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, SighashAllOnly::NAME, item)
            }
            WitnessLayoutUnion::Otx(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, Otx::NAME, item)
            }
            WitnessLayoutUnion::OtxStart(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, OtxStart::NAME, item)
            }
        }
    }
}
impl<'r> ::core::fmt::Display for WitnessLayoutUnionReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            WitnessLayoutUnionReader::SighashAll(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, SighashAll::NAME, item)
            }
            WitnessLayoutUnionReader::SighashAllOnly(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, SighashAllOnly::NAME, item)
            }
            WitnessLayoutUnionReader::Otx(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, Otx::NAME, item)
            }
            WitnessLayoutUnionReader::OtxStart(ref item) => {
                write!(f, "{}::{}({})", Self::NAME, OtxStart::NAME, item)
            }
        }
    }
}
impl WitnessLayoutUnion {
    pub(crate) fn display_inner(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            WitnessLayoutUnion::SighashAll(ref item) => write!(f, "{}", item),
            WitnessLayoutUnion::SighashAllOnly(ref item) => write!(f, "{}", item),
            WitnessLayoutUnion::Otx(ref item) => write!(f, "{}", item),
            WitnessLayoutUnion::OtxStart(ref item) => write!(f, "{}", item),
        }
    }
}
impl<'r> WitnessLayoutUnionReader<'r> {
    pub(crate) fn display_inner(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            WitnessLayoutUnionReader::SighashAll(ref item) => write!(f, "{}", item),
            WitnessLayoutUnionReader::SighashAllOnly(ref item) => write!(f, "{}", item),
            WitnessLayoutUnionReader::Otx(ref item) => write!(f, "{}", item),
            WitnessLayoutUnionReader::OtxStart(ref item) => write!(f, "{}", item),
        }
    }
}
impl ::core::convert::From<SighashAll> for WitnessLayoutUnion {
    fn from(item: SighashAll) -> Self {
        WitnessLayoutUnion::SighashAll(item)
    }
}
impl ::core::convert::From<SighashAllOnly> for WitnessLayoutUnion {
    fn from(item: SighashAllOnly) -> Self {
        WitnessLayoutUnion::SighashAllOnly(item)
    }
}
impl ::core::convert::From<Otx> for WitnessLayoutUnion {
    fn from(item: Otx) -> Self {
        WitnessLayoutUnion::Otx(item)
    }
}
impl ::core::convert::From<OtxStart> for WitnessLayoutUnion {
    fn from(item: OtxStart) -> Self {
        WitnessLayoutUnion::OtxStart(item)
    }
}
impl<'r> ::core::convert::From<SighashAllReader<'r>> for WitnessLayoutUnionReader<'r> {
    fn from(item: SighashAllReader<'r>) -> Self {
        WitnessLayoutUnionReader::SighashAll(item)
    }
}
impl<'r> ::core::convert::From<SighashAllOnlyReader<'r>> for WitnessLayoutUnionReader<'r> {
    fn from(item: SighashAllOnlyReader<'r>) -> Self {
        WitnessLayoutUnionReader::SighashAllOnly(item)
    }
}
impl<'r> ::core::convert::From<OtxReader<'r>> for WitnessLayoutUnionReader<'r> {
    fn from(item: OtxReader<'r>) -> Self {
        WitnessLayoutUnionReader::Otx(item)
    }
}
impl<'r> ::core::convert::From<OtxStartReader<'r>> for WitnessLayoutUnionReader<'r> {
    fn from(item: OtxStartReader<'r>) -> Self {
        WitnessLayoutUnionReader::OtxStart(item)
    }
}
impl WitnessLayoutUnion {
    pub const NAME: &'static str = "WitnessLayoutUnion";
    pub fn as_bytes(&self) -> molecule::bytes::Bytes {
        match self {
            WitnessLayoutUnion::SighashAll(item) => item.as_bytes(),
            WitnessLayoutUnion::SighashAllOnly(item) => item.as_bytes(),
            WitnessLayoutUnion::Otx(item) => item.as_bytes(),
            WitnessLayoutUnion::OtxStart(item) => item.as_bytes(),
        }
    }
    pub fn as_slice(&self) -> &[u8] {
        match self {
            WitnessLayoutUnion::SighashAll(item) => item.as_slice(),
            WitnessLayoutUnion::SighashAllOnly(item) => item.as_slice(),
            WitnessLayoutUnion::Otx(item) => item.as_slice(),
            WitnessLayoutUnion::OtxStart(item) => item.as_slice(),
        }
    }
    pub fn item_id(&self) -> molecule::Number {
        match self {
            WitnessLayoutUnion::SighashAll(_) => 4278190081,
            WitnessLayoutUnion::SighashAllOnly(_) => 4278190082,
            WitnessLayoutUnion::Otx(_) => 4278190083,
            WitnessLayoutUnion::OtxStart(_) => 4278190084,
        }
    }
    pub fn item_name(&self) -> &str {
        match self {
            WitnessLayoutUnion::SighashAll(_) => "SighashAll",
            WitnessLayoutUnion::SighashAllOnly(_) => "SighashAllOnly",
            WitnessLayoutUnion::Otx(_) => "Otx",
            WitnessLayoutUnion::OtxStart(_) => "OtxStart",
        }
    }
    pub fn as_reader<'r>(&'r self) -> WitnessLayoutUnionReader<'r> {
        match self {
            WitnessLayoutUnion::SighashAll(item) => item.as_reader().into(),
            WitnessLayoutUnion::SighashAllOnly(item) => item.as_reader().into(),
            WitnessLayoutUnion::Otx(item) => item.as_reader().into(),
            WitnessLayoutUnion::OtxStart(item) => item.as_reader().into(),
        }
    }
}
impl<'r> WitnessLayoutUnionReader<'r> {
    pub const NAME: &'r str = "WitnessLayoutUnionReader";
    pub fn as_slice(&self) -> &'r [u8] {
        match self {
            WitnessLayoutUnionReader::SighashAll(item) => item.as_slice(),
            WitnessLayoutUnionReader::SighashAllOnly(item) => item.as_slice(),
            WitnessLayoutUnionReader::Otx(item) => item.as_slice(),
            WitnessLayoutUnionReader::OtxStart(item) => item.as_slice(),
        }
    }
    pub fn item_id(&self) -> molecule::Number {
        match self {
            WitnessLayoutUnionReader::SighashAll(_) => 4278190081,
            WitnessLayoutUnionReader::SighashAllOnly(_) => 4278190082,
            WitnessLayoutUnionReader::Otx(_) => 4278190083,
            WitnessLayoutUnionReader::OtxStart(_) => 4278190084,
        }
    }
    pub fn item_name(&self) -> &str {
        match self {
            WitnessLayoutUnionReader::SighashAll(_) => "SighashAll",
            WitnessLayoutUnionReader::SighashAllOnly(_) => "SighashAllOnly",
            WitnessLayoutUnionReader::Otx(_) => "Otx",
            WitnessLayoutUnionReader::OtxStart(_) => "OtxStart",
        }
    }
}

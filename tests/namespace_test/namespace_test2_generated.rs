// automatically generated by the FlatBuffers compiler, do not modify


#![allow(dead_code)]
#![allow(unused_imports)]

use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

pub mod namespace_a {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

pub enum TableInFirstNSOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct TableInFirstNS<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TableInFirstNS<'a> {
    type Inner = TableInFirstNS<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> TableInFirstNS<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TableInFirstNS {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TableInFirstNSArgs<'args>) -> flatbuffers::WIPOffset<TableInFirstNS<'bldr>> {
      let mut builder = TableInFirstNSBuilder::new(_fbb);
      if let Some(x) = args.foo_struct { builder.add_foo_struct(x); }
      if let Some(x) = args.foo_table { builder.add_foo_table(x); }
      builder.add_foo_enum(args.foo_enum);
      builder.finish()
    }

    pub const VT_FOO_TABLE: flatbuffers::VOffsetT = 4;
    pub const VT_FOO_ENUM: flatbuffers::VOffsetT = 6;
    pub const VT_FOO_STRUCT: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn foo_table(&self) -> Option<namespace_b::TableInNestedNS<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<namespace_b::TableInNestedNS<'a>>>(TableInFirstNS::VT_FOO_TABLE, None)
  }
  #[inline]
  pub fn foo_enum(&self) -> namespace_b::EnumInNestedNS {
    self._tab.get::<namespace_b::EnumInNestedNS>(TableInFirstNS::VT_FOO_ENUM, Some(namespace_b::EnumInNestedNS::A)).unwrap()
  }
  #[inline]
  pub fn foo_struct(&self) -> Option<&'a namespace_b::StructInNestedNS> {
    self._tab.get::<namespace_b::StructInNestedNS>(TableInFirstNS::VT_FOO_STRUCT, None)
  }
}

pub struct TableInFirstNSArgs<'a> {
    pub foo_table: Option<flatbuffers::WIPOffset<namespace_b::TableInNestedNS<'a >>>,
    pub foo_enum: namespace_b::EnumInNestedNS,
    pub foo_struct: Option<&'a  namespace_b::StructInNestedNS>,
}
impl<'a> Default for TableInFirstNSArgs<'a> {
    #[inline]
    fn default() -> Self {
        TableInFirstNSArgs {
            foo_table: None,
            foo_enum: namespace_b::EnumInNestedNS::A,
            foo_struct: None,
        }
    }
}
pub struct TableInFirstNSBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TableInFirstNSBuilder<'a, 'b> {
  #[inline]
  pub fn add_foo_table(&mut self, foo_table: flatbuffers::WIPOffset<namespace_b::TableInNestedNS<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<namespace_b::TableInNestedNS>>(TableInFirstNS::VT_FOO_TABLE, foo_table);
  }
  #[inline]
  pub fn add_foo_enum(&mut self, foo_enum: namespace_b::EnumInNestedNS) {
    self.fbb_.push_slot::<namespace_b::EnumInNestedNS>(TableInFirstNS::VT_FOO_ENUM, foo_enum, namespace_b::EnumInNestedNS::A);
  }
  #[inline]
  pub fn add_foo_struct(&mut self, foo_struct: &'b  namespace_b::StructInNestedNS) {
    self.fbb_.push_slot_always::<&namespace_b::StructInNestedNS>(TableInFirstNS::VT_FOO_STRUCT, foo_struct);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TableInFirstNSBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TableInFirstNSBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TableInFirstNS<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum SecondTableInAOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct SecondTableInA<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for SecondTableInA<'a> {
    type Inner = SecondTableInA<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> SecondTableInA<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        SecondTableInA {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args SecondTableInAArgs<'args>) -> flatbuffers::WIPOffset<SecondTableInA<'bldr>> {
      let mut builder = SecondTableInABuilder::new(_fbb);
      if let Some(x) = args.refer_to_c { builder.add_refer_to_c(x); }
      builder.finish()
    }

    pub const VT_REFER_TO_C: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn refer_to_c(&self) -> Option<super::namespace_c::TableInC<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<super::namespace_c::TableInC<'a>>>(SecondTableInA::VT_REFER_TO_C, None)
  }
}

pub struct SecondTableInAArgs<'a> {
    pub refer_to_c: Option<flatbuffers::WIPOffset<super::namespace_c::TableInC<'a >>>,
}
impl<'a> Default for SecondTableInAArgs<'a> {
    #[inline]
    fn default() -> Self {
        SecondTableInAArgs {
            refer_to_c: None,
        }
    }
}
pub struct SecondTableInABuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> SecondTableInABuilder<'a, 'b> {
  #[inline]
  pub fn add_refer_to_c(&mut self, refer_to_c: flatbuffers::WIPOffset<super::namespace_c::TableInC<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<super::namespace_c::TableInC>>(SecondTableInA::VT_REFER_TO_C, refer_to_c);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SecondTableInABuilder<'a, 'b> {
    let start = _fbb.start_table();
    SecondTableInABuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<SecondTableInA<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

}  // pub mod NamespaceA

pub mod namespace_c {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

pub enum TableInCOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct TableInC<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TableInC<'a> {
    type Inner = TableInC<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> TableInC<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TableInC {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TableInCArgs<'args>) -> flatbuffers::WIPOffset<TableInC<'bldr>> {
      let mut builder = TableInCBuilder::new(_fbb);
      if let Some(x) = args.refer_to_a2 { builder.add_refer_to_a2(x); }
      if let Some(x) = args.refer_to_a1 { builder.add_refer_to_a1(x); }
      builder.finish()
    }

    pub const VT_REFER_TO_A1: flatbuffers::VOffsetT = 4;
    pub const VT_REFER_TO_A2: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn refer_to_a1(&self) -> Option<super::namespace_a::TableInFirstNS<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<super::namespace_a::TableInFirstNS<'a>>>(TableInC::VT_REFER_TO_A1, None)
  }
  #[inline]
  pub fn refer_to_a2(&self) -> Option<super::namespace_a::SecondTableInA<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<super::namespace_a::SecondTableInA<'a>>>(TableInC::VT_REFER_TO_A2, None)
  }
}

pub struct TableInCArgs<'a> {
    pub refer_to_a1: Option<flatbuffers::WIPOffset<super::namespace_a::TableInFirstNS<'a >>>,
    pub refer_to_a2: Option<flatbuffers::WIPOffset<super::namespace_a::SecondTableInA<'a >>>,
}
impl<'a> Default for TableInCArgs<'a> {
    #[inline]
    fn default() -> Self {
        TableInCArgs {
            refer_to_a1: None,
            refer_to_a2: None,
        }
    }
}
pub struct TableInCBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TableInCBuilder<'a, 'b> {
  #[inline]
  pub fn add_refer_to_a1(&mut self, refer_to_a1: flatbuffers::WIPOffset<super::namespace_a::TableInFirstNS<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<super::namespace_a::TableInFirstNS>>(TableInC::VT_REFER_TO_A1, refer_to_a1);
  }
  #[inline]
  pub fn add_refer_to_a2(&mut self, refer_to_a2: flatbuffers::WIPOffset<super::namespace_a::SecondTableInA<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<super::namespace_a::SecondTableInA>>(TableInC::VT_REFER_TO_A2, refer_to_a2);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TableInCBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TableInCBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TableInC<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

}  // pub mod NamespaceC


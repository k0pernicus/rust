// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Private types and traits are not allowed in public interfaces.
// This test also ensures that the checks are performed even inside private modules.

#![feature(associated_type_defaults)]
#![deny(private_in_public)]
#![allow(unused)]
#![allow(improper_ctypes)]

mod types {
    struct Priv;
    pub struct Pub;
    pub trait PubTr {
        type Alias;
    }

    pub type Alias = Priv; //~ ERROR private type `types::Priv` in public interface
    //~^ WARNING hard error
    pub enum E {
        V1(Priv), //~ ERROR private type `types::Priv` in public interface
        //~^ WARNING hard error
        V2 { field: Priv }, //~ ERROR private type `types::Priv` in public interface
        //~^ WARNING hard error
    }
    pub trait Tr {
        const C: Priv = Priv; //~ ERROR private type `types::Priv` in public interface
        //~^ WARNING hard error
        type Alias = Priv; //~ ERROR private type `types::Priv` in public interface
        //~^ WARNING hard error
        fn f1(arg: Priv) {} //~ ERROR private type `types::Priv` in public interface
        //~^ WARNING hard error
        fn f2() -> Priv { panic!() } //~ ERROR private type `types::Priv` in public interface
        //~^ WARNING hard error
    }
    extern {
        pub static ES: Priv; //~ ERROR private type `types::Priv` in public interface
        //~^ WARNING hard error
        pub fn ef1(arg: Priv); //~ ERROR private type `types::Priv` in public interface
        //~^ WARNING hard error
        pub fn ef2() -> Priv; //~ ERROR private type `types::Priv` in public interface
        //~^ WARNING hard error
    }
    impl PubTr for Pub {
        type Alias = Priv; //~ ERROR private type `types::Priv` in public interface
        //~^ WARNING hard error
    }
}

mod traits {
    trait PrivTr {}
    pub struct Pub<T>(T);
    pub trait PubTr {}

    pub type Alias<T: PrivTr> = T; //~ ERROR private trait `traits::PrivTr` in public interface
    //~^ WARN trait bounds are not (yet) enforced in type definitions
    //~| WARNING hard error
    pub trait Tr1: PrivTr {} //~ ERROR private trait `traits::PrivTr` in public interface
    //~^ WARNING hard error
    pub trait Tr2<T: PrivTr> {} //~ ERROR private trait `traits::PrivTr` in public interface
        //~^ WARNING hard error
    pub trait Tr3 {
        //~^ ERROR private trait `traits::PrivTr` in public interface
        //~| WARNING hard error
        type Alias: PrivTr;
        fn f<T: PrivTr>(arg: T) {} //~ ERROR private trait `traits::PrivTr` in public interface
        //~^ WARNING hard error
    }
    impl<T: PrivTr> Pub<T> {} //~ ERROR private trait `traits::PrivTr` in public interface
        //~^ WARNING hard error
    impl<T: PrivTr> PubTr for Pub<T> {} //~ ERROR private trait `traits::PrivTr` in public interface
        //~^ WARNING hard error
}

mod traits_where {
    trait PrivTr {}
    pub struct Pub<T>(T);
    pub trait PubTr {}

    pub type Alias<T> where T: PrivTr = T;
        //~^ ERROR private trait `traits_where::PrivTr` in public interface
        //~| WARNING hard error
        //~| WARNING E0122
    pub trait Tr2<T> where T: PrivTr {}
        //~^ ERROR private trait `traits_where::PrivTr` in public interface
        //~| WARNING hard error
    pub trait Tr3 {
        fn f<T>(arg: T) where T: PrivTr {}
        //~^ ERROR private trait `traits_where::PrivTr` in public interface
        //~| WARNING hard error
    }
    impl<T> Pub<T> where T: PrivTr {}
        //~^ ERROR private trait `traits_where::PrivTr` in public interface
        //~| WARNING hard error
    impl<T> PubTr for Pub<T> where T: PrivTr {}
        //~^ ERROR private trait `traits_where::PrivTr` in public interface
        //~| WARNING hard error
}

mod generics {
    struct Priv<T = u8>(T);
    pub struct Pub<T = u8>(T);
    trait PrivTr<T> {}
    pub trait PubTr<T> {}

    pub trait Tr1: PrivTr<Pub> {}
        //~^ ERROR private trait `generics::PrivTr<generics::Pub>` in public interface
        //~| WARNING hard error
    pub trait Tr2: PubTr<Priv> {} //~ ERROR private type `generics::Priv` in public interface
        //~^ WARNING hard error
    pub trait Tr3: PubTr<[Priv; 1]> {} //~ ERROR private type `generics::Priv` in public interface
        //~^ WARNING hard error
    pub trait Tr4: PubTr<Pub<Priv>> {} //~ ERROR private type `generics::Priv` in public interface
        //~^ WARNING hard error
}

mod impls {
    struct Priv;
    pub struct Pub;
    trait PrivTr {
        type Alias;
    }
    pub trait PubTr {
        type Alias;
    }

    impl Priv {
        pub fn f(arg: Priv) {} // OK
    }
    impl PrivTr for Priv {
        type Alias = Priv; // OK
    }
    impl PubTr for Priv {
        type Alias = Priv; // OK
    }
    impl PrivTr for Pub {
        type Alias = Priv; // OK
    }
    impl PubTr for Pub {
        type Alias = Priv; //~ ERROR private type `impls::Priv` in public interface
        //~^ WARNING hard error
    }
}

mod impls_generics {
    struct Priv<T = u8>(T);
    pub struct Pub<T = u8>(T);
    trait PrivTr<T = u8> {
        type Alias;
    }
    pub trait PubTr<T = u8> {
        type Alias;
    }

    impl Priv<Pub> {
        pub fn f(arg: Priv) {} // OK
    }
    impl Pub<Priv> {
        pub fn f(arg: Priv) {} // OK
    }
    impl PrivTr<Pub> for Priv {
        type Alias = Priv; // OK
    }
    impl PubTr<Priv> for Priv {
        type Alias = Priv; // OK
    }
    impl PubTr for Priv<Pub> {
        type Alias = Priv; // OK
    }
    impl PubTr for [Priv; 1] {
        type Alias = Priv; // OK
    }
    impl PubTr for Pub<Priv> {
        type Alias = Priv; // OK
    }
    impl PrivTr<Pub> for Pub {
        type Alias = Priv; // OK
    }
    impl PubTr<Priv> for Pub {
        type Alias = Priv; // OK
    }
}

mod aliases_pub {
    struct Priv;
    mod m {
        pub struct Pub1;
        pub struct Pub2;
        pub struct Pub3;
        pub trait PubTr<T = u8> {
            type Check = u8;
        }
    }

    use self::m::Pub1 as PrivUseAlias;
    use self::m::PubTr as PrivUseAliasTr;
    type PrivAlias = m::Pub2;
    trait PrivTr {
        type AssocAlias;
    }
    impl PrivTr for Priv {
        type AssocAlias = m::Pub3;
    }

    pub fn f1(arg: PrivUseAlias) {} // OK
    pub fn f2(arg: PrivAlias) {} // OK

    pub trait Tr1: PrivUseAliasTr {} // OK
    pub trait Tr2: PrivUseAliasTr<PrivAlias> {} // OK

    impl PrivAlias {
        pub fn f(arg: Priv) {} //~ ERROR private type `aliases_pub::Priv` in public interface
        //~^ WARNING hard error
    }
    // This doesn't even parse
    // impl <Priv as PrivTr>::AssocAlias {
    //     pub fn f(arg: Priv) {} // ERROR private type `aliases_pub::Priv` in public interface
    // }
    impl PrivUseAliasTr for PrivUseAlias {
        type Check = Priv; //~ ERROR private type `aliases_pub::Priv` in public interface
        //~^ WARNING hard error
    }
    impl PrivUseAliasTr for PrivAlias {
        type Check = Priv; //~ ERROR private type `aliases_pub::Priv` in public interface
        //~^ WARNING hard error
    }
    impl PrivUseAliasTr for <Priv as PrivTr>::AssocAlias {
        type Check = Priv; //~ ERROR private type `aliases_pub::Priv` in public interface
        //~^ WARNING hard error
    }
}

mod aliases_priv {
    struct Priv;

    struct Priv1;
    struct Priv2;
    struct Priv3;
    trait PrivTr1<T = u8> {
        type Check = u8;
    }

    use self::Priv1 as PrivUseAlias;
    use self::PrivTr1 as PrivUseAliasTr;
    type PrivAlias = Priv2;
    trait PrivTr {
        type AssocAlias;
    }
    impl PrivTr for Priv {
        type AssocAlias = Priv3;
    }

    pub trait Tr1: PrivUseAliasTr {}
        //~^ ERROR private trait `aliases_priv::PrivTr1` in public interface
        //~| WARNING hard error
    pub trait Tr2: PrivUseAliasTr<PrivAlias> {}
        //~^ ERROR private trait `aliases_priv::PrivTr1<aliases_priv::Priv2>` in public interface
        //~| WARNING hard error
        //~| ERROR private type `aliases_priv::Priv2` in public interface
        //~| WARNING hard error

    impl PrivUseAlias {
        pub fn f(arg: Priv) {} // OK
    }
    impl PrivAlias {
        pub fn f(arg: Priv) {} // OK
    }
    // This doesn't even parse
    // impl <Priv as PrivTr>::AssocAlias {
    //     pub fn f(arg: Priv) {} // OK
    // }
    impl PrivUseAliasTr for PrivUseAlias {
        type Check = Priv; // OK
    }
    impl PrivUseAliasTr for PrivAlias {
        type Check = Priv; // OK
    }
    impl PrivUseAliasTr for <Priv as PrivTr>::AssocAlias {
        type Check = Priv; // OK
    }
}

mod aliases_params {
    struct Priv;
    type PrivAliasGeneric<T = Priv> = T;
    type Result<T> = ::std::result::Result<T, Priv>;

    pub fn f1(arg: PrivAliasGeneric<u8>) {} // OK, not an error
}

fn main() {}

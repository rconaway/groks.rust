#[cfg(test)]
mod tests {
    #[test]
    fn use_a_structure_by_trait() {
        trait Trait {
            fn doit(&self) -> u8;
            fn changeit(&mut self, what:u8);
        }

        struct Foo {
            what: u8,
        }

        impl Trait for Foo {
            fn doit(&self) -> u8 { self.what }
            fn changeit(&mut self, what:u8) { self.what = what; }
        }

        let f1 = Foo { what: 0 };

        // doesn't work because size of f2 not known at runtime
        // let f2:Trait = f1;

        let f2: &Trait = &f1;
        let f2: &Trait = &Foo { what: 0};

        // doesn't work, again because Trait doesn't have constant size
        // fn do_something_with_trait(t:Trait) {}

        fn read_via_trait(t: &Trait) -> u8 { t.doit() }
        let foo = Foo {what: 42};
        assert_eq!(read_via_trait(&foo), 42);

        fn mutate_via_trait(t: &mut Trait) { t.changeit(43); }
        let mut foo = Foo {what: 42};
        mutate_via_trait( &mut foo);
        assert_eq!(read_via_trait(&foo), 43);

    }
}
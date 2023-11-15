pub mod front_of_house {
    pub mod hosting {
        // 公有模块,可以在front_of_house外使用
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }
    }

    mod serving {
        // ]私有模块,只能在front_of_house中使用
        fn take_order() {
            println!("take_order");
        }

        fn serve_order() {
            println!("serve_order");
        }

        fn take_payment() {
            println!("take_payment");
        }
    }
}
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {
        inside::inner_function();
    }
    
    mod inside {
        pub fn inner_function() {
            secret_function();
        }

        fn secret_function() {}
    }
} 

fn try_me() {
    outermost::middle_function();
}
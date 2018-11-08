/**
 * 1) If an item is public, it can be accessed through any of its parent modules
 * 2) If an item is private, it can be accessed only by its immediate parent 
 * module and any of the parent’s child modules
 */

mod outermost {
    pub fn middle_function() {}
    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {
            // 3. What if, in the body of inner_function, you called 
            // ::outermost::middle_secret_function()?
            // 2nd rule will be applied and will be accessible
            ::outermost::middle_secret_function();
        }
        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    // outermost::middle_secret_function();     // function is private
    // outermost::inside::inner_function();     // module is private
    // outermost::inside::secret_function();    // module is private

    // 1. What if the inside module were public?
    // outermost::inside::inner_function();        // would become accessible

    // 2. What if outermost were public and inside were private?
    // nothing would change in the scope of try_me, module would become
    // accessible from parental scope
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

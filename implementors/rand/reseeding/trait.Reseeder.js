(function() {var implementors = {};
implementors["bytes"] = [];
implementors["hyper"] = [];
implementors["libc"] = [];
implementors["mio"] = [];
implementors["openssl"] = [];
implementors["rand"] = [];
implementors["regex_syntax"] = [];
implementors["reqwest"] = [];
implementors["serde"] = [];
implementors["serde_urlencoded"] = [];
implementors["syn"] = [];
implementors["thread_local"] = [];
implementors["tokio_core"] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()

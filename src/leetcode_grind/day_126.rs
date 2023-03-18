// https://leetcode.com/problems/design-browser-history/
// https://leetcode.com/problems/design-browser-history/editorial/
struct BrowserHistory {
    history: Vec<String>,
    current_url: usize,
    last_url: usize,
}
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            history: vec![homepage],
            current_url: 0,
            last_url: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.current_url += 1;
        if self.current_url < self.history.len() {
            self.history[self.current_url] = url;
        } else {
            self.history.push(url);
        }
        self.last_url = self.current_url;
    }

    fn back(&mut self, steps: i32) -> String {
        self.current_url = 0.max(self.current_url as i32 - steps) as usize;
        self.history[self.current_url].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.current_url = self.last_url.min(self.current_url + steps as usize);
        self.history[self.current_url].clone()
    }
}

mod two_stacks {
    struct BrowserHistory {
        history: Vec<String>,
        future: Vec<String>,
        current: String,
    }
    impl BrowserHistory {
        fn new(homepage: String) -> Self {
            Self {
                history: vec![],
                future: vec![],
                current: homepage,
            }
        }

        fn visit(&mut self, url: String) {
            self.history.push(self.current.clone());
            self.future = vec![];
            self.current = url;
        }

        fn back(&mut self, mut steps: i32) -> String {
            while steps > 0 && !self.history.is_empty() {
                steps -= 1;
                self.future.push(self.current.clone());
                self.current = self.history.pop().unwrap();
            }
            self.current.clone()
        }

        fn forward(&mut self, mut steps: i32) -> String {
            while steps > 0 && !self.future.is_empty() {
                steps -= 1;
                self.history.push(self.current.clone());
                self.current = self.future.pop().unwrap();
            }
            self.current.clone()
        }
    }
}

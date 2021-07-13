#[derive(Default)]
struct CQueue {
    q: Vec<i32>,
    k: Vec<i32>,
}

impl CQueue {
    fn new() -> Self {
        Default::default()
    }

    fn append_tail(&mut self, value: i32) {
        self.q.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        match self.k.pop() {
            Some(n) => n,
            None => {
                self.q.reverse();
                self.k.append(&mut self.q);
                self.k.pop().unwrap_or(-1)
            }
        }
    }
}


// python3
//class CQueue:
//    def __init__(self):
//        self.A = []
//        self.B = []
//
//    def appendTail(self, value: int) -> None:
//        self.A.append(value)
//
//    def deleteHead(self) -> int:
//        if self.B:
//            return self.B.pop()
//        if not self.A:
//            return -1
//        while self.A:
//            self.B.append(self.A.pop())
//        return self.B.pop()
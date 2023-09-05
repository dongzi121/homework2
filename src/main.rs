struct Buffer<T> {
    data: Vec<T>,
}

impl<T> Buffer<T> {
    // 创建一个新的Buffer实例
    fn new() -> Self {
        Buffer { data: Vec::new() }
    }

    // 添加一个元素到Buffer
    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    // 计算所有成员的和
    fn sum(&self) -> T
    where
        T: std::ops::Add<Output = T> + Default + Clone,
    {
        self.data.iter().cloned().fold(T::default(), |acc, val| acc + val)
    }
}

struct Buffer2<T>{
    data: Vec<T>,
}
impl<T> Buffer2<T>{
    fn new() -> Self {
        Buffer2 { data: Vec::new() }
    }

    fn push(){
        self.data
    }
}

fn compare_string(x: &str, y: &str) -> bool {
    let mut x_chars = x.chars();
    let mut y_chars = y.chars();

    loop {
        match (x_chars.next(), y_chars.next()) {
            (Some(x_char), Some(y_char)) => {
                if x_char > y_char {
                    return true;
                } else if x_char < y_char {
                    return false;
                }
                // 如果当前字符相等，继续比较下一个字符
            }
            (Some(_), None) => {
                // y 达到末尾，但 x 还有字符，x 更长，所以 x > y
                return true;
            }
            (None, Some(_)) => {
                // x 达到末尾，但 y 还有字符，y 更长，所以 x < y
                return false;
            }
            (None, None) => {
                // x 和 y 都达到末尾，相等
                return false;
            }
        }
    }
}

fn main() {
    let mut buffer = Buffer::new();
    buffer.push(1);
    buffer.push(2);
    buffer.push(3);

    let sum = buffer.sum();
    println!("Sum: {:?}", sum); // 输出 Sum: 6
    ///////////////
    let x = "apple";
    let y = "banana";

    let result = compare_string(x, y);

    if result {
        println!("'{}' > '{}'", x, y);
    } else {
        println!("'{}' <= '{}'", x, y);
    }
    ///////////////////
    let original_vec: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let new_vec: Vec<char> = original_vec.iter().skip(1).map(|&c| c).chain(vec!['f']).collect(); 
    println!("{:?}", new_vec);
}

pub mod solution {

    #[derive(Debug)]
    pub struct CircularBuffer <T> {
        pub capacity : usize,
        pub array : Box<[Option<T>]>,
        pub head : usize,
        pub tail : usize,
        pub size : usize
    }


    #[derive(Debug, PartialEq, Eq)]
    pub struct BufferFullError;


    impl <T> CircularBuffer <T> 
        where T: Clone
    {


        pub fn new ( capacity : usize ) -> Self { 
            let mut vec = Vec::with_capacity(capacity);
            
            for _ in 0..capacity {
                vec.push(None);
            }

            CircularBuffer {
                capacity: capacity,
                array: vec.into_boxed_slice() ,
                head: 0,
                tail: 0,
                size : 0
            }
        }

        pub fn write (&mut self, item: T) -> Result<(), BufferFullError> {

            if self.capacity == self.size {
                return Err(BufferFullError);
            }

            self.array[self.tail] = Some(item);

            self.tail = (self.tail + 1) % self.capacity;
            
            self.size += 1;

            Ok(())

        }


        pub fn read (&mut self) -> Option <T> { 

            if self.size == 0{
                return None;
            }

            let value = self.array[self.head].clone();
            let _ = self.array[self.head] = None;
            self.head = (self.head + 1) % self.capacity;

            self.size -= 1;

            value
            

        }

        pub fn clear (&mut self) {
            
            self.head = 0;
            self.tail = 0;

            for i in 0..self.size {
                self.array[i] = None
            }

            self.size = 0;

            

        }

        pub fn size(&self) -> usize { 
            
            self.size

         }

        // // Scrive forzando la s o v r a s c r i t t u r a dell ’ elemento piu ’ vecchio
        pub fn overwrite (&mut self, item : T) { 
        
            self.array[self.head] = Some(item);

         }
        // R i o r g a n i z z a il buffer ren de nd ol o contiguo in memoria
        //pub fn make_conti g u o u s (& mut self ) { todo !() }
    }


    // fn main(){

    //     let mut a = CircularBuffer::<i32>::new(5);

    //     match a.read() {
    //         Some(value) => println!("Value {} Vec {:?}", value, a),
    //         _ => println!("cacato"),
    //     }

    //     let _ = a.write(5);

    //     match a.write(6){
    //         Ok(()) => println!("{:?}", a),
    //         _ => println!("casotto"),
    //     }

    //     match a.read() {
    //         Some(value) => println!("Value {} Vec {:?}", value, a),
    //         _ => println!("cacato"),
    //     }


    // }


}
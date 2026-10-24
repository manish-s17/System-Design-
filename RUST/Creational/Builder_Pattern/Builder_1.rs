

 #[derive(Debug)]
   struct People{
        name :String,
        id : i32,
        email :String,
        description :String,
   }


 impl People{
          fn new() -> Self{
                   Self{ 
                        name : "".to_string(),
                        id : 0,
                        email : "".to_string(),
                        description :"".to_string(),
                        }
               }
      } 


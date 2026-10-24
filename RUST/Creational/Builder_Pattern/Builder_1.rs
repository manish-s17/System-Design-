

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

           fn name(mut self ,name :String ) -> Self{
                 self.name =  name;
                 self
           }

            fn id(mut self , id : i32 ) -> Self{
                self.id = id;
                self
            }

            fn email(mut self , email : String) -> Self{
                 self.email = email;
                 self
            }

            fn description(mut self, des :String ) -> Self{
                     self.description = des ;
                      self
            }
      } 


 fn main(){
       println!("Hellow World!");

       let p:People = People::new()
               .name("Manish Sah".to_string())
               .id(000932)
               .email("manishsahcnb@gmail.com".to_string())
               .description("Hellow i am manish sah and i am a rust developer ".to_string());
            
         println!("{:?}",p);

         dbg!(p);
 }

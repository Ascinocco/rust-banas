mod pizza_order {
  pub struct Pizza {
    pub dough: String,
    pub cheese: String,
    pub topping: String,
  }

  impl Pizza {
    pub fn lunch(lt: &str) -> Pizza {
      Pizza {
        dough: String::from("Regular dough"),
        cheese: String::from("Moz"),
        topping: String::from(lt),
      }
    }
  }

  pub mod help_customer{
    fn seat_at_table() {
      println!("Customer Steated at table");
    }

    pub fn take_order() {
      seat_at_table();
      
      let cust_pizza: super::Pizza = super::Pizza::lunch("Veggies");
      serve_customer(cust_pizza);
    }

    fn serve_customer(cust_pizza: super::Pizza) {
      println!("The customer is server a regular pizza with {}", cust_pizza.topping);
    }
  }
}

pub fn order_food() {
  crate::restaurant::pizza_order::help_customer::take_order();
}
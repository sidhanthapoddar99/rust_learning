
struct dimension{
    dim : f64,
    unit : String
}

struct circle {
    radius : dimension,
}

impl circle {

    fn area(&mut self) -> f64
    {
        let area : f64 = 3.14 * self.radius.dim * self.radius.dim;
        area
    }

    fn compare_circle(&self, cir : &circle) -> bool
    {
        self.radius.dim >= cir.radius.dim
    }
}

struct rectange {
    length : dimension,
    breath : dimension,
}




fn main() {
    
    let rec: rectange = rectange {
            length : dimension{dim: 97.5, unit : String::from("cm")},
            breath : dimension{dim: 88.2, unit: String::from("cm")}
    };


    let area: f64 = calculate_area(&rec);

    println!("The are of rectangle = {} X {} = {}", rec.length.dim, rec.breath.dim, area);

    // in the above case we calculate the are of the rectangle our method calculate_area is very 
    // closely related to the rectangle struct but is defined outside the struct. 
    // We can define the method inside the struct as well using implentation block

    // ----------------- Defining the method inside the struct -----------------

    let mut cir: circle = circle {
        radius : dimension{dim: 5.0, unit : String::from("cm")}
    };


    // ----------------- Calling the method of the struct -----------------

    let area: f64 = cir.area();

    println!("The are of circle = {} X {} = {}", cir.radius.dim, cir.radius.dim, area);

    // ----------------- Comparing the two circles -----------------

    let cir2: circle = circle {
        radius : dimension{dim: 4.0, unit : String::from("cm")}
    };

    let result: bool = cir.compare_circle(&cir2);

    if result {
        println!("The first circle is greater than or equal to the second circle");
    } else {
        println!("The first circle is less than the second circle");
    }


}


fn calculate_area(rec : &rectange) -> f64
{

    let area: f64 = rec.length.dim * rec.breath.dim;
    area

}
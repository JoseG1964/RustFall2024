#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}
#[derive(Debug)]
enum Major {
    ComputerScience,
    ElectricalEngineering,
}
#[derive(Debug)]
struct Student {
    name:String,
    grade:GradeLevel,
    major:Major
}

impl Student {
    fn new(name:String,grade:GradeLevel,major:Major) -> Self {
        Student {
            name:name,
            grade:grade,
            major:major,
        }
    }

    fn introduce_yourself(&self) {
        //TODO! (implement printing info about Student
        println!("Hi my name is {}.", self.name);
        // use match statement)
        match self.grade {
            GradeLevel::Bachelor => println!("Im pursuing a Bachelors."),
            GradeLevel::Master => println!("Im pursuing a Masters."),
            GradeLevel::PhD => println!("Im pursing a PhD."),
        }
        
        match self.major{
            Major::ComputerScience => println!("My major is Computer Science."),
            Major::ElectricalEngineering => println!("My major is Electrical Engineering."),
        }

        }
    }


fn main() {
    let s1 = Student::new("Jose".to_string(),
    GradeLevel::Bachelor,
    Major::ComputerScience);
s1.introduce_yourself();
}
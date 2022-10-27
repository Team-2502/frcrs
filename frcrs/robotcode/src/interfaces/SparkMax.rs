pub trait SparkMax {
    //! bind to a SparkMax at the given CAN address
    pub fn bind(id: u16) -> impl SparkMax;
    //! stops the motor
    pub fn disable(); 
    //! match the speed of another SparkMax
    pub fn follow(leader: impl SparkMax, inverted: bool);
    //! returns CAN id of SparkMax
    pub fn getID() -> u16;
    //! returns setpoint of controller (velocity, position, etc)
    pub fn get() -> f64;
    //! returns the encoder of the Spark's attached motor
    pub fn getEncoder() -> impl RelativeEncoder;
    //! returns the encoder on the Spark's encoder port
    pub fn getAltEncoder() -> impl RelativeEncoder;
    //! returns the temperature of the attached motor, in C
    pub fn getTemp() -> f64;
    
}

pub trait RelativeEncoder {}

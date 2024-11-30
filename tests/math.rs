use rrtk_proc::*;
#[test]
fn math_test() {
    fn add(x: u8, y: u8) -> u8 {
        x + y
    }
    fn sub(x: u8, y: u8) -> u8 {
        x - y
    }
    fn mul(x: u8, y: u8) -> u8 {
        x * y
    }
    fn div(x: u8, y: u8) -> u8 {
        x / y
    }
    let kp = 20;
    let ki = 1;
    let kd = 5;
    let error = 5;
    let int = 10;
    let drv = 2;
    //math!(kp * error + ki * integral(error) + kd * derivative(error));
    //math!(kp * rrtk::streams::control::EWMAStream::<f32, E>::new(error.clone(), 5.0) + 1.0);
    math!(kp * error + ki * int + kd * drv);
    //math!(x_n * a + x_p * (1 - a));
}

use rrtk_proc::*;
#[test]
fn math_test() {
    //math!(kp * error + ki * integral(error) + kd * derivative(error));
    //math!(kp * rrtk::streams::control::EWMAStream::<f32, E>::new(error.clone(), 5.0) + 1.0);
    math!(kp * error + ki * int + kd * drv);
    //math!(x_n * a + x_p * (1 - a));
}

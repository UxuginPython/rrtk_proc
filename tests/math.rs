use rrtk_proc::*;
use rrtk::Getter;
#[test]
fn math_test() {
    /*fn add(x: u8, y: u8) -> u8 {
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
    }*/
    let time = rrtk::rc_ref_cell_reference(rrtk::Time::default());
    let kp = rrtk::rc_ref_cell_reference(rrtk::ConstantGetter::<u8, rrtk::Time, ()>::new(time.clone(), 20));
    let ki = rrtk::rc_ref_cell_reference(rrtk::ConstantGetter::<u8, rrtk::Time, ()>::new(time.clone(), 1));
    let kd = rrtk::rc_ref_cell_reference(rrtk::ConstantGetter::<u8, rrtk::Time, ()>::new(time.clone(), 5));
    let error = rrtk::rc_ref_cell_reference(rrtk::ConstantGetter::<u8, rrtk::Time, ()>::new(time.clone(), 5));
    let int = rrtk::rc_ref_cell_reference(rrtk::ConstantGetter::<u8, rrtk::Time, ()>::new(time.clone(), 10));
    let drv = rrtk::rc_ref_cell_reference(rrtk::ConstantGetter::<u8, rrtk::Time, ()>::new(time.clone(), 2));
    let pid = math!(kp * error + ki * int + kd * drv);
    assert_eq!(pid.borrow().get().unwrap().unwrap().value, 120);
}

use rrtk::*;
use rrtk_proc::*;
#[test]
fn math_test() {
    let time = rc_ref_cell_reference(Time::default());
    let kp = rc_ref_cell_reference(ConstantGetter::<u8, Time, ()>::new(time.clone(), 20));
    let ki = rc_ref_cell_reference(ConstantGetter::<u8, Time, ()>::new(time.clone(), 1));
    let kd = rc_ref_cell_reference(ConstantGetter::<u8, Time, ()>::new(time.clone(), 5));
    let error = rc_ref_cell_reference(ConstantGetter::<u8, Time, ()>::new(time.clone(), 5));
    let int = rc_ref_cell_reference(ConstantGetter::<u8, Time, ()>::new(time.clone(), 10));
    let drv = rc_ref_cell_reference(ConstantGetter::<u8, Time, ()>::new(time.clone(), 2));
    let pid = math!(kp * error + ki * int + kd * drv);
    assert_eq!(pid.borrow().get().unwrap().unwrap().value, 120);
}

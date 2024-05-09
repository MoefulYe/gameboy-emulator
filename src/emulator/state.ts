/*
shutdown -- 点击开机按钮 --> running
running -- 点击暂停按钮 --> paused
running -- 运行出错 --> aborted
running -- 点击关机按钮 --> shutdown
paused -- 点击继续按钮 --> running
paused -- 单步执行出错 --> aborted
paused -- 点击关机按钮 --> shutdown
aborted -- 点击重启按钮 --> running
aborted -- 点击关机按钮 --> shutdown
*/
export const enum EmulatorState {
  // 关机状态，模拟器未启动
  Shutdown,
  // 正常运行状态
  Running,
  // 暂停正常模拟器执行, 但是可以进行单步调试
  Paused,
  // 模拟器出现错误，进入锁定状态，此时只能重启模拟器
  Aborted
}

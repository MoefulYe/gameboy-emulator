/*
shutdown -- 开机 --> running
running -- 暂停 --> paused
running -- 出错 --> aborted
running -- 关机 --> shutdown
paused -- 继续 --> running
paused -- 出错 --> aborted
paused -- 关机 --> shutdown
aborted -- 重置 --> shutdown
aborted -- 关机 --> shutdown
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

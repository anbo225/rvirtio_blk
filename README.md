# rvirtio_blk
A study project to develop virtio_blk driver using Rust.


## Reference 

定义设备相关的数据结构，包括设备信息、设备状态、设备操作标识等

设备初始化，即完成对设备的初始配置，分配I/O操作所需的内存，设置好中断处理例程

如果设备会产生中断，需要有处理这个设备中断的中断处理例程（Interrupt Handler）

根据操作系统上层模块（如文件系统）的要求（如读磁盘数据），给I/O设备发出命令，检测和处理设备出现的错误

与操作系统上层模块或应用进行交互，完成上层模块或应用的要求（如上传读出的磁盘数据）

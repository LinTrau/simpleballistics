这是一个基于 Rust 实现的高性能动力学模拟引擎，专注于弹道计算、刚体及多体动力学模拟。项目采用 Nix 进行环境管理，并计划通过 FFI 或数据接口与 
Julia (Makie) 协同工作，实现复杂任务的数值模拟与可视化。

## 1. 项目介绍

当前版本已实现基础的**质点动力学 (Particle Dynamics)** 模拟，支持：
- 考虑重力与空气阻力的三维空间运动模拟。
- 基于 `nalgebra` 的高效向量运算。
- 模拟数据的 CSV 序列化输出。
- 基于 Nix 的高度可复现开发环境。

## 2. 快速开始

### 使用 Nix (推荐)
如果你使用 NixOS 或已安装 Nix，可以直接进入隔离的开发环境：

```bash
# 进入开发环境 (包含 Rust 工具链, Julia, OpenBLAS 等)
nix develop

# 或者直接构建项目
nix build

```

### 使用 Cargo

确保已安装 Rust 工具链：

```bash
# 运行模拟
cargo run --release

# 结果将保存至 trajectory.csv

```

## 3. 开发路线图 (To-Do List)

### 第一阶段：软件质量与健壮性 (P0 - 紧急)

* [x] **基础框架**：实现质点运动学步进与力学模型。
* [ ] **错误处理**：将 `main.rs` 中的 `unwrap` 或简单 `Box<dyn Error>` 替换为 `thiserror` 或 
`anyhow`，处理物理参数非法（如质量为负）的边界情况。
* [ ] **单元测试**：
* `physics.rs`：验证重力与阻力合成向量的正确性。
* `engine.rs`：验证积分算法（如欧拉法）在简单情形下的解析解对齐。


* [ ] **参数化配置**：引入 `serde` 与 `toml`，将环境参数（重力、密度）与弹药参数从代码中抽离到 `config.toml`。

### 第二阶段：动力学模型扩展 (P1 - 重要)

* [ ] **高级积分器**：引入辛积分算法（Symplectic Integrators，如 Verlet 或 Runge-Kutta 
4th），以满足长时间动力学模拟的能量守恒。
* [ ] **刚体动力学**：
* 引入四元数（Quaternion）处理姿态解算。
* 实现转动惯量张量计算与力矩（Torque）模型。


* [ ] **多体动力学 (MBD)**：
* 定义约束（Constraints）与铰链（Joints）。
* 建立多体系统的拓扑结构表达。

### 第三阶段：Julia 协同与可视化 (P2 - 增强)

* [ ] **Julia 接口层**：
* 使用 `ccall` 或 `PyO3/inline-python` 风格的绑定，暴露 Rust 核心计算函数给 Julia。
* 或者通过共享内存/零拷贝序列化（如 Apache Arrow）进行高速数据传递。


* [ ] **Makie 可视化**：
* 编写 Julia 脚本读取 Rust 实时输出的 State 流。
* 实现基于 Makie 的三维轨迹实时渲染与动态动画演示。


* [ ] **复杂外弹道任务**：在 Julia 端调用 Rust 引擎进行蒙特卡洛弹道散布分析。

---

## 4. 目录结构

* `src/domain.rs`: 核心数据结构定义（状态、环境、配置）。
* `src/physics.rs`: 力学模型实现。
* `src/engine.rs`: 数值积分与状态步进引擎。
* `src/main.rs`: 模拟主循环与数据落盘。
* `flake.nix`: Nix Flake 环境配置。




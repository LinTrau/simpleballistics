这是一个基于 Rust 实现的高性能动力学模拟引擎，专注于弹道计算、刚体及多体动力学模拟。项目通过 Nix 提供一致的开发环境，并计划与 Julia 
(Makie) 协同进行高性能数值模拟与可视化。

## 1. 项目介绍与部署

### 开发环境依赖
本项目推荐使用 **Nix** 进行环境管理。通过项目根目录下的 `flake.nix` 或 `shell.nix`，你可以一键获取 Rust 
工具链、Julia 及其相关物理计算库（OpenBLAS 等）。

#### 部署步骤
1. **克隆仓库**：
   ```bash
   git clone 
[https://github.com/LinTrau/simpleballistics.git](https://github.com/LinTrau/
simpleballistics.git)
   cd simpleballistics

```

2. **进入环境**：
* 如果已启用 Flakes：`nix develop`
* 如果使用传统 Nix：`nix-shell`


3. **编译与运行**：
```bash
cargo run --release

```

## 2. 任务路线图 (To-Do List)

### 第一阶段：工程基础与可靠性 (P0 - 当前重点)

* [x] **核心框架**：建立质点动力学基础状态方程与步进引擎。
* [x] **环境配置**：完成 Nix 隔离环境与 Git 仓库初始化。
* [ ] **错误处理**：引入 `anyhow` 替换现有的 `Box<dyn Error>`。
* [ ] **物理校验单元测试**：
* 验证 `physics::total_force` 在真空（空气密度为 0）下的抛物线解析解对齐情况。
* 验证 `engine::step` 在极端小步长下的收敛性。



### 第二阶段：模型深化与刚体动力学 (P1 - 物理层扩展)

* [ ] **参数化配置系统**：实现通过外部 TOML 文件加载弹药参数（质量、阻力系数）与环境参数（气压、重力场模型）。
* [ ] **六自由度 (6-DOF) 扩展**：
* 将 `State` 扩展至包含角动量与姿态四元数。
* 实现基于转动惯量张量的力矩解算。


* [ ] **高阶积分器**：实现四阶龙格-库塔法 (RK4) 或辛积分算法以降低长程模拟误差。

### 第三阶段：Julia 协同与高级模拟 (P2 - 交互层)

* [ ] **Julia 接口暴露**：通过数据管道或 FFI 允许 Julia 调用 Rust 编写的高性能物理核心。
* [ ] **Makie 可视化**：编写 Julia 脚本实现实时 3D 弹道轨迹渲染与刚体姿态动画展示。
* [ ] **多体动力学 (MBD)**：在 Rust 层实现多体约束求解器，支撑复杂机械系统的数值模拟。

## 3. 目录说明

* `src/domain.rs`: 定义 `State`, `Environment`, `ProjectileConfig` 等核心域模型。
* `src/physics.rs`: 处理重力、空气阻力等物理矢量计算。
* `src/engine.rs`: 负责状态转移方程与数值步进。
* `src/main.rs`: 模拟入口，负责主循环与 CSV 数据落盘。

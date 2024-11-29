mod actor_units;
mod agent;
mod environments;
use crate::agent::{Action, Agent};
use crate::environments::{RaidEnv, SimulationEnvironment};
use plotters::prelude::*;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // コマンドライン引数を取得し、エージェントを初期化
    let args: Vec<String> = env::args().collect();
    let mut agent: Agent = args.into();

    // 環境の初期化
    let mut env = RaidEnv::new(7)?;

    // エポック数
    let num_epochs = 1000;

    // 各エポックの報酬を格納するベクタ
    let rewards: Vec<f32> = (0..num_epochs)
        .map(|_| {
            let mut player = env.raider;
            let mut total_reward = 0.0;

            // 1エポック内で最大30ステップ実行
            for _ in 0..30 {
                // 動次の行動候補を取得
                let next_actions = agent.get_next_actions(&player);

                // エージェントが行動を選択
                let action = agent.choose_action(player, &next_actions);

                // 環境の状態を更新し、次の状態と報酬を取得
                let (next_player, reward) = env.step(action);

                // Qテーブルを更新
                agent.update_q_table(player, action, reward, next_player, &next_actions);

                // 探索率を減衰
                agent.decay_exploration();

                // 報酬を加算し、プレイヤーの状態を更新
                total_reward += reward;
                player = next_player;

                // 終了条件：ラインアウト
                if env.court.is_out_of_bounds(player) {
                    break;
                }
            }

            total_reward
        })
        .collect();

    // 結果をプロット
    plot_results(&rewards)?;

    Ok(())
}

fn plot_results(rewards: &[f32]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("q_learning_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Q-Learning Rewards", ("sans-serif", 50))
        .build_cartesian_2d(0..rewards.len(), -1.0..1.0)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        (0..).zip(rewards.iter().map(|&r| r as f64)),
        &RED,
    ))?;

    Ok(())
}

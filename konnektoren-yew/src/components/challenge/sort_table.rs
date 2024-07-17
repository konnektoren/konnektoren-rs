use crate::components::challenge::ChallengeEvent;
use konnektoren_core::challenges::{ChallengeResult, SortTable, SortTableRow};
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, Element, TouchEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct SortTableComponentProps {
    pub challenge: SortTable,
    #[prop_or_default]
    pub on_finish: Option<Callback<ChallengeResult>>,
    #[prop_or_default]
    pub on_event: Option<Callback<ChallengeEvent>>,
}

fn shuffle(rows: &Vec<SortTableRow>) -> Vec<SortTableRow> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    let mut rng = thread_rng();

    let mut rows = rows.clone();

    let mut columns: Vec<Vec<String>> = vec![vec![]; rows[0].values.len()];

    for row in &rows {
        for (col_idx, value) in row.values.iter().enumerate() {
            columns[col_idx].push(value.clone());
        }
    }

    for col in 1..columns.len() {
        columns[col].shuffle(&mut rng);
    }

    for (row_idx, row) in rows.iter_mut().enumerate() {
        for col_idx in 1..columns.len() {
            row.values[col_idx] = columns[col_idx][row_idx].clone();
        }
    }

    rows
}

#[function_component(SortTableComponent)]
pub fn sort_table_comp(props: &SortTableComponentProps) -> Html {
    let SortTableComponentProps {
        challenge,
        on_finish,
        on_event,
    } = props;

    let rows = use_state(|| shuffle(&challenge.rows));
    let dragged_cell = use_state(|| None::<(usize, usize)>);
    let selected_cell = use_state(|| None::<(usize, usize)>);

    let handle_drag_start = {
        let dragged_cell = dragged_cell.clone();
        Callback::from(move |(row_index, col_index): (usize, usize)| {
            dragged_cell.set(Some((row_index, col_index)));
        })
    };

    let handle_drag_over = Callback::from(|event: DragEvent| {
        event.prevent_default();
    });

    let handle_touch_start = {
        let dragged_cell = dragged_cell.clone();
        Callback::from(move |event: TouchEvent| {
            event.prevent_default();
            let target = event.target().unwrap();
            let element = target.dyn_into::<Element>().unwrap();
            let row_index = element
                .get_attribute("data-row-index")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let col_index = element
                .get_attribute("data-col-index")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            dragged_cell.set(Some((row_index, col_index)));
        })
    };

    let handle_touch_move = {
        let rows = rows.clone();
        let dragged_cell = dragged_cell.clone();
        Callback::from(move |event: TouchEvent| {
            event.prevent_default();
            if let Some(touch) = event.touches().get(0) {
                let target = touch.target().unwrap();
                let element = target.dyn_into::<Element>().unwrap();
                let target_row_index = element
                    .get_attribute("data-row-index")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let target_col_index = element
                    .get_attribute("data-col-index")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                if let Some((source_row_index, source_col_index)) = *dragged_cell {
                    let mut updated_rows = (*rows).clone();

                    if source_row_index != target_row_index || source_col_index != target_col_index
                    {
                        if source_col_index < updated_rows[source_row_index].values.len()
                            && target_col_index < updated_rows[target_row_index].values.len()
                        {
                            let src_value = updated_rows[source_row_index]
                                .values
                                .remove(source_col_index);
                            let tgt_value = updated_rows[target_row_index]
                                .values
                                .remove(target_col_index);

                            updated_rows[source_row_index]
                                .values
                                .insert(source_col_index, tgt_value);
                            updated_rows[target_row_index]
                                .values
                                .insert(target_col_index, src_value);

                            rows.set(updated_rows);
                        }
                    }
                }
            }
        })
    };

    let handle_drop = {
        let rows = rows.clone();
        let dragged_cell = dragged_cell.clone();
        Callback::from(
            move |(event, target_row_index, target_col_index): (DragEvent, usize, usize)| {
                event.prevent_default();
                if let Some((source_row_index, source_col_index)) = *dragged_cell {
                    let mut updated_rows = (*rows).clone();

                    if source_row_index != target_row_index || source_col_index != target_col_index
                    {
                        if source_col_index < updated_rows[source_row_index].values.len()
                            && target_col_index < updated_rows[target_row_index].values.len()
                        {
                            let src_value = updated_rows[source_row_index]
                                .values
                                .remove(source_col_index);
                            let tgt_value = updated_rows[target_row_index]
                                .values
                                .remove(target_col_index);

                            updated_rows[source_row_index]
                                .values
                                .insert(source_col_index, tgt_value);
                            updated_rows[target_row_index]
                                .values
                                .insert(target_col_index, src_value);

                            rows.set(updated_rows);
                        }
                    }
                }
            },
        )
    };

    let handle_cell_click = {
        let rows = rows.clone();
        let selected_cell = selected_cell.clone();
        Callback::from(move |(row_index, col_index): (usize, usize)| {
            if let Some((selected_row_index, selected_col_index)) = *selected_cell {
                if selected_row_index != row_index || selected_col_index != col_index {
                    let mut updated_rows = (*rows).clone();

                    if selected_col_index < updated_rows[selected_row_index].values.len()
                        && col_index < updated_rows[row_index].values.len()
                    {
                        let src_value = updated_rows[selected_row_index]
                            .values
                            .remove(selected_col_index);
                        let tgt_value = updated_rows[row_index].values.remove(col_index);

                        updated_rows[selected_row_index]
                            .values
                            .insert(selected_col_index, tgt_value);
                        updated_rows[row_index].values.insert(col_index, src_value);

                        rows.set(updated_rows);
                    }
                }
                selected_cell.set(None);
            } else {
                selected_cell.set(Some((row_index, col_index)));
            }
        })
    };

    let handle_finish = {
        let on_finish = on_finish.clone();
        let on_event = on_event.clone();
        let rows = rows.clone();
        Callback::from(move |_| {
            let result = ChallengeResult::SortTable((*rows).clone());
            if let Some(on_finish) = on_finish.as_ref() {
                on_finish.emit(result.clone());
            }
            if let Some(on_event) = on_event.as_ref() {
                on_event.emit(ChallengeEvent::Finish(result));
            }
        })
    };

    html! {
        <div class="sort-table">
            <h1>{ &challenge.name }</h1>
            <p>{ &challenge.description }</p>
            <table>
                <thead>
                    <tr>
                        { for challenge.columns.iter().map(|column| html! { <th>{ column.title.to_string() }</th> }) }
                    </tr>
                </thead>
                <tbody>
                    { for rows.iter().enumerate().map(|(row_index, row)| html! {
                        <tr>
                            { for row.values.iter().enumerate().map(|(col_index, value)| {
                                let is_selected = match *selected_cell {
                                    Some((selected_row, selected_col)) => selected_row == row_index && selected_col == col_index,
                                    None => false,
                                };
                                html! {
                                    <td
                                        class={if is_selected { "selected" } else { "" }}
                                        draggable="true"
                                        data-row-index={row_index.to_string()}
                                        data-col-index={col_index.to_string()}
                                        ondragstart={handle_drag_start.reform(move |_| (row_index, col_index))}
                                        ondragover={handle_drag_over.clone()}
                                        ondrop={handle_drop.reform(move |event| (event, row_index, col_index))}
                                        ontouchstart={handle_touch_start.clone()}
                                        ontouchmove={handle_touch_move.clone()}
                                        onclick={handle_cell_click.reform(move |_| (row_index, col_index))}
                                    >
                                        { value }
                                    </td>
                                }
                            }) }
                        </tr>
                    }) }
                </tbody>
            </table>
            <button onclick={handle_finish}>{"Finish"}</button>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        SortTableComponent,
        SortTableComponentProps {
            challenge: SortTable::default(),
            ..Default::default()
        },
    );
}

import { Board as WasmBoard } from "wasm-minesweeper";

const canvas = document.getElementById("board");
const canvas_ctx = canvas.getContext('2d');
const board_face = document.getElementById("board-face");
const board_flag = document.getElementById("board-flag");
let Board = null;

function reset() {
    const CELL_SIZE = 18;
    let row = 30;
    let column = 20;
    let total_bombs = 100;

    if (screen.width <= 480) {
        row = 15;
        column = 10;
        total_bombs = 25;
    } else if (screen.width <= 720) {
        row = 25;
        column = 15;
        total_bombs = 70;
    }

    Board = WasmBoard.new(row, column, CELL_SIZE, total_bombs);
}

function initialize_board() {
    canvas.height = Board.get_row() * Board.get_cell_size();
    canvas.width = Board.get_column() * Board.get_cell_size();
}

function initialize_hooks() {
    canvas.addEventListener("click", canvas_click_event);
    canvas.addEventListener("contextmenu", canvas_right_click_event);
    board_face.addEventListener("click", reset);
}

function initialize() {
    reset();
    initialize_board();
    initialize_hooks();
}

function find_cell_coordinates_from_event(raw_event) {
    const client_boundary = canvas.getBoundingClientRect();
    const cell_size = Board.get_cell_size();
    const y_coordinate = Math.floor((raw_event.clientY - client_boundary.top) / cell_size);
    const x_coordinate = Math.floor((raw_event.clientX - client_boundary.left) / cell_size);

    return [x_coordinate, y_coordinate];
}

function canvas_click_event(raw_event) {
    const coordinates = find_cell_coordinates_from_event(raw_event);

    Board.reveal_cell(coordinates[0], coordinates[1]);
    // 0 -> Pristine
    // 1 -> Undecided
    // 2 -> Won
    // 3 -> Lost
    const board_state = Board.get_state();
    if (board_state == 2) {
        alert("You Won!");
        reset();
    }
}

function canvas_right_click_event(raw_event) {
    raw_event.preventDefault();
    const coordinates = find_cell_coordinates_from_event(raw_event);
    Board.toggle_flag(coordinates[0], coordinates[1]);
}

function event_loop() {
    board_flag.innerText = Board.get_total_flags_left();
    if (Board.get_state() == 3) {
        board_face.style.backgroundImage = "url('./face_sad.png')";
    } else {
        board_face.style.backgroundImage = "url('./face_happy.png')";
    }

    let cells_details = Board.flat_cells_details();
    for (let i = 0; i < cells_details.length; i += 7) {

        let x_pos = cells_details[i];
        let y_pos = cells_details[i + 1];
        let is_bomb = cells_details[i + 2];
        let total_bomb = cells_details[i + 3];
        let is_revealed = cells_details[i + 4];
        let flaged = cells_details[i + 5];
        let bomb_triggered = cells_details[i + 6];
        let cell_size = Board.get_cell_size();


        canvas_ctx.beginPath();
        canvas_ctx.rect(x_pos, y_pos, cell_size, cell_size);
        canvas_ctx.strokeStyle = '#ffffff';
        canvas_ctx.stroke();

        canvas_ctx.textAlign = "center";
        canvas_ctx.textBaseline = "middle";
        canvas_ctx.font = "10px monospace";
        canvas_ctx.shadowColor = "black";
        canvas_ctx.shadowBlur = 4;

        if (flaged) {
            canvas_ctx.fillStyle = "#bdbdbd";
            canvas_ctx.fill();
            canvas_ctx.fillText("🚩", x_pos + (cell_size / 2), cells_details[i + 1] + (cell_size / 2));
        } else if (!is_revealed) {
            canvas_ctx.fillStyle = "#bdbdbd";
            canvas_ctx.fill();
        } else if (is_bomb) {
            canvas_ctx.shadowBlur = 1;
            canvas_ctx.fillStyle = "white";
            if (bomb_triggered) {
                canvas_ctx.fillStyle = "#e95e5e";
            }
            canvas_ctx.fill();
            canvas_ctx.fillText("💣", x_pos + (cell_size / 2), cells_details[i + 1] + (cell_size / 2));
        } else {
            canvas_ctx.shadowBlur = 1;
            canvas_ctx.fillStyle = "white";
            canvas_ctx.fill();

            if (total_bomb == 0) { continue; }

            canvas_ctx.fillStyle = ["", "#0000ff", "#008000", "#ff0000", "#000080", "#800000", "#008080", "#000000", "#808080"][total_bomb];
            canvas_ctx.font = "15px monospace";
            canvas_ctx.fillText(total_bomb, x_pos + (cell_size / 2), cells_details[i + 1] + (cell_size / 2));
        }
    }

    requestAnimationFrame(event_loop);
}

function main() {
    initialize();
    event_loop();
}

main();
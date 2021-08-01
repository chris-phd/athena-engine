// Dynamic import of the rust wasm code
async function main() {
    console.log("Importing the rust wasm code");
    const lib = await import("../pkg/index.js").catch(console.error);

    setupPage();
}

main();

// Webpage setup
function setupPage() {
    console.log("setupPage");

    // Setup the chess pieces
    setupChessBoardSquares();

    // Assign the squares the correct event handlers
    var squares = document.getElementsByClassName("square");
    for (var i = 0; i < squares.length; ++i) {
        squares[i].ondragover = onDragOver;
        squares[i].ondrop = onDrop;
    }

    // Set the chess pieces
    startingPositionFen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    setBoardFromFenString(startingPositionFen);

    console.log("bottom of setup page");
}

// Piece event handlers
function onDragStart(event) {
    event.dataTransfer.setData("text/html", event.target.id);
}

// Square event handlers
function onDragOver(event) {
    event.preventDefault();
}

function onDrop(event) {
    event.preventDefault();
    var id = event.dataTransfer.getData("text/html");

    // Check if this move was a capture
    var square;
    if (isCaptureMove(event.target)) {
        var capturedPiece = event.target;
        square = event.target.parentElement;
        square.removeChild(capturedPiece);
    } else {
        square = event.target;
    }

    square.appendChild(document.getElementById(id));
}

function isCaptureMove(htmlElement) {
    // Does the id match the format of a piece id
    if (htmlElement.id.length == 5 && isAlpha(htmlElement.id[0]) && isAlpha(htmlElement.id[1])) {
        return true;
    }

    return false;
}

// Chess board setup functions
function setupChessBoardSquares() {
    
    // Set the chess pieces from the top left square (a8)
    var colourInx = 0;
    for (var rank = 8; rank >= 1; --rank) {

        rankElement = document.createElement("div");
        rankElement.id = "rank-".concat(rank.toString());
        rankElement.className = "rank";

        for (var file = 1; file <= 8; ++file) {

            squareElement = document.createElement("div");
            squareElement.id = squareFromRankAndFile(rank, file)

            if (colourInx%2 == 0) {
                squareElement.className = "light square";
            } else {
                squareElement.className = "dark square";
            }

            rankElement.appendChild(squareElement);

            ++colourInx;
        }

        chessBoardElement = document.getElementById("chess-board");
        chessBoardElement.appendChild(rankElement);

        ++colourInx;
    }
}

function setBoardFromFenString(fenString) {
    // fenBoardString describes a board position using Forsyth-Edwarsd notation.
    // https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation

    // Read the fen string and set the corresponding board position.
    // Begin setting the board in the top left square (a8)
    var rank = 8;
    var file = 1;
    for (var i = 0; i < fenString.length; ++i) {
        if (isDigit(fenString[i])) {
            file += fenString[i] - '0';
        } else if (fenString[i] == '/') {
            rank -= 1;
            file = 1;
        } else if (isAlpha(fenString[i])) {
            setPiece(fenString[i], rank, file);
            file += 1; 
        } else if (fenString[i] == ' ') {
            break;
        } else {
            console.log("%s is an unrecognised character", fenString[i]);
        }
    }
}

function setPiece(pieceAsFenChar, rank, file) {
    // pieceAsFenChar, a single character descrbing a chess piece. 
    // upper case for white pieces, lower case for black pieces.

    squareCode = squareFromRankAndFile(rank, file);

    let pieceColour = "white"; // upper
    if (isLowerChar(pieceAsFenChar)) {
        pieceColour = "black"; // lower
        pieceAsFenChar = pieceAsFenChar.toUpperCase();
    }

    
    var htmlId = pieceColour[0].concat(pieceAsFenChar).concat('-').concat(squareCode);
    var hmtlClass = "piece ";
    if (pieceAsFenChar == 'K') {
        hmtlClass = hmtlClass.concat(pieceColour).concat("-king");
    } else if (pieceAsFenChar == 'Q') {
        hmtlClass = hmtlClass.concat(pieceColour).concat("-queen");
    } else if (pieceAsFenChar == 'B') {
        hmtlClass = hmtlClass.concat(pieceColour).concat("-bishop");
    } else if (pieceAsFenChar == 'N') {
        hmtlClass = hmtlClass.concat(pieceColour).concat("-knight");
    } else if (pieceAsFenChar == 'R') {
        hmtlClass = hmtlClass.concat(pieceColour).concat("-rook");
    } else if (pieceAsFenChar == 'P') {
        hmtlClass = hmtlClass.concat(pieceColour).concat("-pawn");
    } else {
        console.log("Invalid character piece: %s", pieceAsFenChar);
    }

    let pieceElement = document.createElement("div");
    pieceElement.className = hmtlClass;
    pieceElement.id = htmlId;
    pieceElement.draggable = true;
    pieceElement.ondragstart = onDragStart;

    square = document.getElementById(squareCode);
    if (square != null) {
        square.appendChild(pieceElement);
    } else {
        console.log("Could not set %s at square %s. Square not found.", htmlId, squareCode);
    }

}

// Helpers

function squareFromRankAndFile(rank, file) {
    if (rank < 1 || rank > 8 || file < 1 || file > 8) {
        console.log("rank = &d, file = %d", rank, file);
        throw "Invalid rank or file!";
    }

    let ranks = ['1', '2', '3', '4', '5', '6', '7', '8'];
    let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    return files[file-1].concat(ranks[rank-1]);
}

function isDigit(c) {
    return c >= '0' && c <= '9';
}

function isAlpha(c) {
    return isUpperChar(c) || isLowerChar(c);
}

function isUpperChar(c) {
    return c >= 'A' && c <= 'Z';
}

function isLowerChar(c) {
    return c >= 'a' && c <= 'z';
}
$(document).ready(function() {
  console.log("READY");
  let current_player = 0;
  let socket = new WebSocket("ws://127.0.0.1:8080");
  let cmd = { command: "Start" };

  socket.onopen = function(e) {
    let json = JSON.stringify(cmd);
    console.log(json);
    socket.send(json);
  };

  socket.onmessage = function(msg) {
    console.log(msg.data);

    if (msg.data.includes("Game Start")) {
      let slices = msg.data.split(":");
      current_player = slices[1];

      if (current_player == 1) {
        $("#selection").addClass("player1");
      } else {
        $("#selection").addClass("player2");
      }
    } else if (msg.data.includes("Winner")) {
      alert("Winner: Player " + current_player);
      cmd.command = "Reset";
      let json = JSON.stringify(cmd);
      socket.send(json);
    } else if (msg.data.includes("Resetted")) {
      reset_board();
    }
  };

  socket.onerror = function(error) {
    console.log("Error: ", error);
  };

  $("#selection").on("click", ".row .cell", function() {
    dropToken(parseInt($(this).index()));
  });

  function reset_board() {
    let $children = $("#gameBoard").children();
    $($children).each(function(i,child) {
      $(child).children().each(function(j, td) {
       if ($(td).hasClass("player1")) {
        $(td).removeClass("player1");
       }
       if ($(td).hasClass("player2")) {
        $(td).removeClass("player2");
       }
      })
    });
  }
  function dropToken(col) {
    let $children = $("#gameBoard").children();
    for (var i = $children.length - 1; i >= 0; --i) {
      console.log(i);
      // $($($($children)[row]).children())[col]
      if (
        $($($($($children)[i]).children())[col]).hasClass("player1") ||
        $($($($($children)[i]).children())[col]).hasClass("player2")
      ) {
        console.log("yes");
      } else {
        if (current_player == 1) {
          $($($($($children)[i]).children())[col]).addClass("player1");
          current_player = 2;
          $("#selection").removeClass("player1");
          $("#selection").addClass("player2");
        } else {
          $($($($($children)[i]).children())[col]).addClass("player2");
          current_player = 1;
          $("#selection").removeClass("player2");
          $("#selection").addClass("player1");
        }
        cmd.command = "Set Move:" + col;
        let json = JSON.stringify(cmd);
        socket.send(json);
        return;
      }
    }
  }
});

$(document).ready(function() {
    if(is_touch_device()) {
        alert("You are playing the game on the backend server. This is a basic interface not meant to be played on touch devices.\n" +
        "This is intended behavior. Various frontends properly designed are planned");
        alert("Left click is short touch. Right click is long touch");
    }
    window.X_COORDINATES = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"];
    window.ORIENTATIONS = ["N", "E", "S", "W"];
    window.current_orientation = 0;
    window.PLANE_SHAPES = {
        "N": [[-2,  1], [-1,  1], [ 0,  1], [ 1,  1], [ 2,  1], [ 0,  2], [-1,  3], [ 0,  3], [ 1,  3]],
        "S": [[ 2, -1], [ 1, -1], [ 0, -1], [-1, -1], [-2, -1], [ 0, -2], [ 1, -3], [ 0, -3], [-1, -3]],
        "E": [[-1, -2], [-1, -1], [-1,  0], [-1,  1], [-1,  2], [-2,  0], [-3, -1], [-3,  0], [-3,  1]],
        "W": [[ 1,  2], [ 1,  1], [ 1,  0], [ 1, -1], [ 1, -2], [ 2,  0], [ 3,  1], [ 3,  0], [ 3, -1]],
    }
    window.get_tile_coordinates = function($elem) {
        var this_x = $elem.prevAll("td").length;
        var tile_letter = window.X_COORDINATES[this_x];
        var this_y = $elem.parent().parent().children().index($elem.parent());
        var tile_number = this_y+1;
        return {
            x: this_x,
            y: this_y,
            letter: tile_letter,
            number: tile_number,
        };
    }
    window.get_tile_by_coord = function($grid, x, y) {
        var rows = $grid.find("tbody tr");
        if(y >= rows.length) {
            return false;
        }
        var columns = $(rows[y]).find("td");
        if(x >= columns.length) {
            return false
        }
        return $(columns[x])
    }
    window.get_plane_tiles_from_head = function($grid, $head, orientation) {
        var head_data = window.get_tile_coordinates($head);
        var tiles = [];
        for(var i = 0; i < window.PLANE_SHAPES[orientation].length; i++) {
            var deltas = window.PLANE_SHAPES[orientation][i];
            var tile = window.get_tile_by_coord($grid, head_data.x + deltas[0], head_data.y + deltas[1]);
            if(tile != false) {
                tiles.push(tile);
            }
        }
        return tiles;
    }
    window.highlight_plane_tiles = function(data, orientation) {
        var className = "highlighted-temp-tile";
        if (data.className) {
            className = data.className;
        }
        data.tile.addClass(className);
        tiles = get_plane_tiles_from_head(data.grid, data.tile, orientation);
        for(var i=0; i<tiles.length; i++) {
            tiles[i].addClass(className);
        }
    }
    window.unhighlight_plane_tiles = function(data, orientation) {
        var className = "highlighted-temp-tile";
        if (data.className) {
            className = data.className;
        }
        data.tile.removeClass(className);
        tiles = get_plane_tiles_from_head(data.grid, data.tile, orientation);
        for(var i=0; i<tiles.length; i++) {
            tiles[i].removeClass(className);
        }
    }

    // game initialization
    if ($("#new_head").length == 1 && $("#new_orientation").length == 1) {

        $("#new_head").data("prev", "");
        $("#new_orientation").data("prev", "");
        $("#new_head").val("");
        $("#new_orientation").val("");

        $("#own_board tbody td").on("mouseover", function() {
            data = window.get_tile_coordinates($(this));
            data.grid = $("#own_board");
            data.tile = $(this);
            data.grid.trigger("tileover", data);
        });
        $("#own_board tbody td").on("mouseout", function() {
            data = window.get_tile_coordinates($(this));
            data.grid = $("#own_board");
            data.tile = $(this);
            data.grid.trigger("tileout", data);
        });
        $("#own_board tbody td").on("contextmenu", function(e) {
            e.preventDefault();
            data = window.get_tile_coordinates($(this));
            data.grid = $("#own_board");
            data.tile = $(this);
            data.grid.trigger("planerotate", data);
        });
        $("#own_board tbody td").on("click", function(e) {
            data = window.get_tile_coordinates($(this));
            data.grid = $("#own_board");
            data.tile = $(this);
            data.orientation = window.ORIENTATIONS[window.current_orientation % window.ORIENTATIONS.length];
            data.grid.trigger("planeplaced", data);
        });

        $("#own_board").on("tileover", function(ev, data) {
            window.highlight_plane_tiles(data, window.ORIENTATIONS[window.current_orientation % window.ORIENTATIONS.length ]);
        });
        $("#own_board").on("tileout", function(ev, data) {
            window.unhighlight_plane_tiles(data, window.ORIENTATIONS[window.current_orientation % window.ORIENTATIONS.length ]);
        });
        $("#own_board").on("planerotate", function(e, data) {
            window.unhighlight_plane_tiles(data, window.ORIENTATIONS[window.current_orientation % window.ORIENTATIONS.length ]);
            window.current_orientation = ++window.current_orientation % window.ORIENTATIONS.length;
            window.highlight_plane_tiles(data, window.ORIENTATIONS[window.current_orientation % window.ORIENTATIONS.length ]);
        });
        $("#own_board").on("planeplaced", function(ev, data) {
            var $new_head = $("#new_head");
            var $new_orientation = $("#new_orientation");
            var $grid = $(this);

            var prev_head = $new_head.data("prev");
            var prev_orientation = $new_orientation.data("prev");
            if (prev_head && prev_orientation) {
                var letter = prev_head.substring(0, 1);
                var number = parseInt(prev_head.substring(1));
                var x = window.X_COORDINATES.indexOf(letter);
                var y = number-1;
                var prev_data = {
                    x: x,
                    y: y,
                    letter: letter,
                    number: number,
                    orientation: prev_orientation,
                    grid: $grid,
                    tile: get_tile_by_coord($grid, x, y),
                };
                if (prev_data.tile) {
                    prev_data.className = "highlighted-fixed-tile";
                    window.unhighlight_plane_tiles(prev_data, prev_orientation);
                }
            }

            $new_head.val(data.letter + data.number);
            $new_orientation.val(data.orientation);

            $new_head.data("prev", $new_head.val());
            $new_orientation.data("prev", $new_orientation.val());

            data.className = "highlighted-fixed-tile";
            window.highlight_plane_tiles(data, data.orientation);
            $("#send_to_mission").css('visibility', 'visible');
        });
    } else { // scrapbook interaction
        $("#own_scrapbook tbody td").on("mouseover", function() {
            data = window.get_tile_coordinates($(this));
            data.grid = $("#own_scrapbook");
            data.tile = $(this);
            data.grid.trigger("tileover", data);
        });
        $("#own_scrapbook tbody td").on("mouseout", function() {
            data = window.get_tile_coordinates($(this));
            data.grid = $("#own_scrapbook");
            data.tile = $(this);
            data.grid.trigger("tileout", data);
        });
        $("#own_scrapbook tbody td").on("click", function(e) {
            data = window.get_tile_coordinates($(this));
            data.grid = $("#own_scrapbook");
            data.tile = $(this);
            data.orientation = window.ORIENTATIONS[window.current_orientation % window.ORIENTATIONS.length];
            data.grid.trigger("tilehit", data);
        });

        $("#own_scrapbook").on("tileover", function(ev, data) {
            data.tile.addClass("highlighted-temp-hit");
        });
        $("#own_scrapbook").on("tileout", function(ev, data) {
            data.tile.removeClass("highlighted-temp-hit");
        });
        $("#own_scrapbook").on("tilehit", function(ev, data) {
            $("#new_hit").val(data.letter + data.number);
            $("#bombard_form").submit();
        });
    }
})
function is_touch_device() {
  return 'ontouchstart' in window || navigator.maxTouchPoints;
};

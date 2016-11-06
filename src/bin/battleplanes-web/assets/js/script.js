$(document).ready(function(){
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
        data.tile.addClass("highlighted-temp-tile");
        tiles = get_plane_tiles_from_head(data.grid, data.tile, orientation);
        for(var i=0; i<tiles.length; i++) {
            tiles[i].addClass("highlighted-temp-tile");
        }
    }
    window.unhighlight_plane_tiles = function(data, orientation) {
        data.tile.removeClass("highlighted-temp-tile");
        tiles = get_plane_tiles_from_head(data.grid, data.tile, orientation);
        for(var i=0; i<tiles.length; i++) {
            tiles[i].removeClass("highlighted-temp-tile");
        }
    }

    if ($("#new_head").length == 1 && $("#new_orientation").length == 1) {
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
            $("#new_head").val(data.letter + data.number);
            $("#new_orientation").val(data.orientation);
        });
    } else {
        console.log("switching mode");
    }
})

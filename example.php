<?php

start_server(function ($query) {
    parse_str($query, $output);

    return "First param: " . $output['first'] .  ". Second param: " . $output['second'] . ".";
});

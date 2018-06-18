function set_path_length () {
    let path = $('.recaman-svg-path');
    let length = path[0].getTotalLength();

    path.css('stroke-dasharray', length);
    path.css('stroke-dashoffset', length);

    $.keyframe.define([{
        'name': 'animator',
        '0%': {'stroke-dashoffset': length},
        '75%': {'stroke-dashoffset': 0},
        '100%': {'stroke-dashoffset': 0},
    }]);
}

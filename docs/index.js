function set_path_length () {
    let path = document.querySelector('.recaman-svg-path');
    let length = path.getTotalLength();

    path.style['stroke-dasharray'] = length;
    path.style['stroke-dashoffset'] = length;
}

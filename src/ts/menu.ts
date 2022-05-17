export function rightClick(e) {
	e.preventDefault();
	document.getElementById('contextMenu').style.display = 'block';
	var menu = document.getElementById('contextMenu');
	menu.style.display = 'block';
	menu.style.left = e.pageX + 'px';
	menu.style.top = e.pageY + 'px';
	return e.srcElement.id;
}
export function hideMenu() {
	document.getElementById('contextMenu').style.display = 'none';
}

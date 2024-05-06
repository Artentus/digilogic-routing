const anchorMouseEnter = (target) => {
    target.setAttribute("stroke", "lawngreen");
    document.querySelectorAll(".neighbor-of-" + target.id).forEach(
        (neighbor) => { neighbor.setAttribute("stroke", "lawngreen"); }
    );
};

const anchorMouseLeave = (target) => {
    target.setAttribute("stroke", "none");
    document.querySelectorAll(".neighbor-of-" + target.id).forEach(
        (neighbor) => { neighbor.setAttribute("stroke", "none"); }
    );
};

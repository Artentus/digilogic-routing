const INVALID_BOUNDING_BOX_INDEX: u32 = u32::MAX;

const ANCHORS: &[Anchor] = &[
    Anchor {
        position: Point { x: 215, y: 270 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 290, y: 270 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 215, y: 330 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 290, y: 330 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 280, y: 300 },
        bounding_box: 0,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 215, y: 330 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 290, y: 330 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 215, y: 390 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 290, y: 390 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 280, y: 360 },
        bounding_box: 1,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 215, y: 550 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 290, y: 550 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 215, y: 610 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 290, y: 610 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 280, y: 580 },
        bounding_box: 2,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 215, y: 10 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 290, y: 10 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 215, y: 70 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 290, y: 70 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 280, y: 40 },
        bounding_box: 3,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 889, y: 550 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 964, y: 550 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 889, y: 610 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 964, y: 610 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 900, y: 580 },
        bounding_box: 4,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 509, y: 330 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 584, y: 330 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 509, y: 410 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 584, y: 410 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 520, y: 360 },
        bounding_box: 5,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 520, y: 380 },
        bounding_box: 5,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 574, y: 370 },
        bounding_box: 5,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 509, y: 250 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 584, y: 250 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 509, y: 330 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 584, y: 330 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 520, y: 280 },
        bounding_box: 6,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 520, y: 300 },
        bounding_box: 6,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 574, y: 290 },
        bounding_box: 6,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 509, y: 410 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 584, y: 410 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 509, y: 490 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 584, y: 490 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 520, y: 440 },
        bounding_box: 7,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 520, y: 460 },
        bounding_box: 7,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 574, y: 450 },
        bounding_box: 7,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 689, y: 430 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 764, y: 430 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 689, y: 510 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 764, y: 510 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 700, y: 460 },
        bounding_box: 8,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 700, y: 480 },
        bounding_box: 8,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 754, y: 470 },
        bounding_box: 8,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 689, y: 510 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 764, y: 510 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 689, y: 590 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 764, y: 590 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 700, y: 540 },
        bounding_box: 9,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 700, y: 560 },
        bounding_box: 9,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 754, y: 550 },
        bounding_box: 9,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 789, y: 530 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 864, y: 530 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 789, y: 610 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 864, y: 610 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 800, y: 560 },
        bounding_box: 10,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 800, y: 580 },
        bounding_box: 10,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 854, y: 570 },
        bounding_box: 10,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 329, y: 330 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 404, y: 330 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 329, y: 410 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 404, y: 410 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 340, y: 360 },
        bounding_box: 11,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 340, y: 380 },
        bounding_box: 11,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 394, y: 370 },
        bounding_box: 11,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 215, y: 370 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 290, y: 370 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 215, y: 430 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 290, y: 430 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 280, y: 400 },
        bounding_box: 12,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 1829, y: 310 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1904, y: 310 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1829, y: 370 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1904, y: 370 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1840, y: 340 },
        bounding_box: 13,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1729, y: 290 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1804, y: 290 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1729, y: 370 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1804, y: 370 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1740, y: 320 },
        bounding_box: 14,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1740, y: 340 },
        bounding_box: 14,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1794, y: 330 },
        bounding_box: 14,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 1589, y: 170 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1664, y: 170 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1589, y: 250 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1664, y: 250 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1600, y: 200 },
        bounding_box: 15,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1600, y: 220 },
        bounding_box: 15,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1654, y: 210 },
        bounding_box: 15,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 1449, y: 110 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1524, y: 110 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1449, y: 190 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1524, y: 190 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1460, y: 140 },
        bounding_box: 16,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1460, y: 160 },
        bounding_box: 16,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1514, y: 150 },
        bounding_box: 16,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 1289, y: 130 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1364, y: 130 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1289, y: 210 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1364, y: 210 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1300, y: 160 },
        bounding_box: 17,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1300, y: 180 },
        bounding_box: 17,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1354, y: 170 },
        bounding_box: 17,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 1449, y: 230 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1524, y: 230 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1449, y: 310 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1524, y: 310 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1460, y: 260 },
        bounding_box: 18,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1460, y: 280 },
        bounding_box: 18,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1514, y: 270 },
        bounding_box: 18,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 1289, y: 250 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1364, y: 250 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1289, y: 330 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1364, y: 330 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1300, y: 280 },
        bounding_box: 19,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1300, y: 300 },
        bounding_box: 19,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1354, y: 290 },
        bounding_box: 19,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 1589, y: 410 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1664, y: 410 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1589, y: 490 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1664, y: 490 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1600, y: 440 },
        bounding_box: 20,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1600, y: 460 },
        bounding_box: 20,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1654, y: 450 },
        bounding_box: 20,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 1449, y: 350 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1524, y: 350 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1449, y: 430 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1524, y: 430 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1460, y: 380 },
        bounding_box: 21,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1460, y: 400 },
        bounding_box: 21,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1514, y: 390 },
        bounding_box: 21,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 1289, y: 370 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1364, y: 370 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1289, y: 450 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1364, y: 450 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1300, y: 400 },
        bounding_box: 22,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1300, y: 420 },
        bounding_box: 22,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1354, y: 410 },
        bounding_box: 22,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 1449, y: 470 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1524, y: 470 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1449, y: 550 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1524, y: 550 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1460, y: 500 },
        bounding_box: 23,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1460, y: 520 },
        bounding_box: 23,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1514, y: 510 },
        bounding_box: 23,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 1289, y: 490 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1364, y: 490 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1289, y: 570 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1364, y: 570 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1300, y: 520 },
        bounding_box: 24,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1300, y: 540 },
        bounding_box: 24,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1354, y: 530 },
        bounding_box: 24,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 1029, y: 10 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1104, y: 10 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1029, y: 70 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1104, y: 70 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1040, y: 40 },
        bounding_box: 25,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1094, y: 40 },
        bounding_box: 25,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 1149, y: -30 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1224, y: -30 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1149, y: 30 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1224, y: 30 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1160, y: 0 },
        bounding_box: 26,
        connect_directions: Directions::NEG_X,
    },
    Anchor {
        position: Point { x: 1214, y: 0 },
        bounding_box: 26,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 215, y: -30 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 290, y: -30 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 215, y: 30 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 290, y: 30 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 280, y: 0 },
        bounding_box: 27,
        connect_directions: Directions::POS_X,
    },
    Anchor {
        position: Point { x: 1140, y: 0 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 480, y: 580 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1220, y: 200 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 660, y: 460 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 480, y: 400 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1020, y: 280 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1100, y: 160 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1020, y: 40 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 500, y: 360 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 500, y: 300 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 1140, y: 440 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 640, y: 380 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
    Anchor {
        position: Point { x: 480, y: 380 },
        bounding_box: INVALID_BOUNDING_BOX_INDEX,
        connect_directions: Directions::ALL,
    },
];

const BOUNDING_BOXES: &[BoundingBox] = &[
    BoundingBox {
        center: Point { x: 253, y: 300 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: 253, y: 360 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: 253, y: 580 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: 253, y: 40 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: 927, y: 580 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: 547, y: 370 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 547, y: 290 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 547, y: 450 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 727, y: 470 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 727, y: 550 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 827, y: 570 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 367, y: 370 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 253, y: 400 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: 1867, y: 340 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: 1767, y: 330 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 1627, y: 210 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 1487, y: 150 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 1327, y: 170 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 1487, y: 270 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 1327, y: 290 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 1627, y: 450 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 1487, y: 390 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 1327, y: 410 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 1487, y: 510 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 1327, y: 530 },
        half_width: 36,
        half_height: 39,
    },
    BoundingBox {
        center: Point { x: 1067, y: 40 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: 1187, y: 0 },
        half_width: 36,
        half_height: 29,
    },
    BoundingBox {
        center: Point { x: 253, y: 0 },
        half_width: 36,
        half_height: 29,
    },
];

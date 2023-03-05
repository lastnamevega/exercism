ORDINALS = [
    'first',
    'second',
    'third',
    'fourth',
    'fifth',
    'sixth',
    'seventh',
    'eighth',
    'ninth',
    'tenth',
    'eleventh',
    'twelfth',
]

LYRICS = [
    'a Partridge in a Pear Tree.',
    'two Turtle Doves, and ',
    'three French Hens, ',
    'four Calling Birds, ',
    'five Gold Rings, ',
    'six Geese-a-Laying, ',
    'seven Swans-a-Swimming, ',
    'eight Maids-a-Milking, ',
    'nine Ladies Dancing, ',
    'ten Lords-a-Leaping, ',
    'eleven Pipers Piping, ',
    'twelve Drummers Drumming, ',
]


def generate_verse(number):
    ordinal = ORDINALS[number - 1]
    verse = f'On the {ordinal} day of Christmas my true love gave to me: '

    for i in range(number, 0, -1):
        verse = f'{verse}{LYRICS[i - 1]}'

    return verse


def recite(start, stop):
    return [generate_verse(i) for i in range(start, stop + 1)]

object ResistorColor {
    enum class Colors {
        black,
        brown,
        red,
        orange,
        yellow,
        green,
        blue,
        violet,
        grey,
        white
    }

    fun colorCode(input: String): Int = Colors.valueOf(input).ordinal

    fun colors(): List<String> = Colors.values().map { color -> color.name }
}

class DndCharacter {

    val strength: Int = ability()
    val dexterity: Int = ability()
    val constitution: Int = ability()
    val intelligence: Int = ability()
    val wisdom: Int = ability()
    val charisma: Int = ability()
    val hitpoints: Int = 10 + modifier(constitution)

    companion object {

        private fun roll() = (1..6).random()

        fun ability(): Int {
            val rolls = List(4) { roll() }
            return rolls.sorted().drop(0).sum()
        }

        fun modifier(score: Int) = score / 2 - 5
    }

}

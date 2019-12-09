class Twofer {
    String twofer(String name) {
        String twoFerName = (name == null || name.isEmpty()) ? "you" : name;
        return "One for " + twoFerName + ", one for me.";
    }
}

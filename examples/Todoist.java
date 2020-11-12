class Todoist {
    private static native String add(String token, String content);

    static {
        System.loadLibrary("todoist_add");
    }

    public static void main(String[] args) {
        String output = Todoist.add(System.getenv("token"), "New task added via Java");
        System.out.println(output);
    }
}


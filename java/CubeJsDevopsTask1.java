import java.nio.file.Files;
import java.nio.file.Paths;

// Main class should be named 'Solution'
class Solution {
    public static final String SRC = "/root/customers/data.csv";

    public static void main(String[] args) throws Exception {
        long count = Files.readAllLines(Paths.get(SRC)).stream().skip(1)
            .map(s -> s.split(",")[0]).distinct().count();
        System.out.println("Total customers:");
        System.out.println(count);
    }
}
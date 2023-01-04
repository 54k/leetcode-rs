import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Objects;
import java.util.regex.Pattern;
import java.util.stream.Stream;

// Main class should be named 'Solution'
class Solution {
    public static final String SRC = "/root/devops/";

    public static void main(String[] args) throws Exception {
        var pattern = "(?i)(\\d{1,3}\\.\\d{1,3}\\.\\d{1,3}\\.\\d{1,3})";
        var r = Pattern.compile(pattern);

        try (Stream<Path> items = Files.walk(Paths.get(SRC))) {
            items.filter(Files::isRegularFile).flatMap((path) -> {
                    try {
                        return Files.readAllLines(path).stream().flatMap(line -> {
                            var m = r.matcher(line);
                            var res = new ArrayList<String>();
                            while (m.find()) {
                                res.add(m.group(1));
                            }
                            return res.stream();
                        }).filter(Objects::nonNull);
                    } catch (Exception ignore) {
                        return Stream.empty();
                    }
                }).distinct()
                .filter(s -> Arrays.stream(s.split("\\.")).map(Integer::valueOf).allMatch(i -> i >= 0 && i <= 255))
                .sorted().forEach(System.out::println);
        }
    }
}
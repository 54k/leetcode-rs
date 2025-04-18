package leetcode_grind;

import java.util.regex.*;

public class Day881 {
    public String countAndSay(int n) {
        String currentString = "1";
        Pattern pattern = Pattern.compile("(.)\\1*");
        for (int i = 1; i < n; i++) {
            Matcher m = pattern.matcher(currentString);
            StringBuffer nextString = new StringBuffer();
            while (m.find()) {
                nextString.append(m.group().length() + String.valueOf(m.group().charAt(0)));
            }
            currentString = nextString.toString();
        }
        return currentString;
    }
}

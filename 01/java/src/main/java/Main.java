import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;

public class Main {

    public static void main(String[] args) throws IOException {
        BufferedReader reader = new BufferedReader(new FileReader("src/main/java/input"));
        System.out.println((Long) reader.lines()
                .map(Integer::parseInt)
                .mapToLong(CalcFuel::CalcRecursive)
                .sum());
    }
}

public class CalcFuel {

    public static int Calc(int mass) {
        return ((int) mass / 3) - 2;
    }

    public static int CalcRecursive(int moduleMass) {
        int fuel = CalcFuel.Calc(moduleMass);
        return fuel <= 0 ? 0 : fuel + CalcRecursive(fuel);
    }

}

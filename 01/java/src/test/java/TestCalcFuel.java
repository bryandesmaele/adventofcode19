import org.junit.Assert;
import org.junit.Test;

public class TestCalcFuel {
    @Test
     public void testCalc(){
         int[] inputs = {12, 14, 100756, 1969};
         int[] outputs = {2, 2, 33583, 654};

         for(int i =0; i<inputs.length;i++){
             Assert.assertEquals(CalcFuel.Calc(inputs[i]), outputs[i]);
         }
     }

     @Test
    public void testCalcRecursive(){
         int[] inputs = {12, 1969, 100756};
         int[] outputs = {2, 966, 50346};

         for(int i =0; i<inputs.length;i++){
             Assert.assertEquals(outputs[i],CalcFuel.CalcRecursive(inputs[i]));
         }
     }
}

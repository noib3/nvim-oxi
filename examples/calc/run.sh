export name="calc"
export name_lib="libcalc"

add="add(-1, 128)"
mul="multiply(-1, 128)"
cpt="compute(calc.multiply, 0, 128)"
export lua="
calc=require'calc' add=calc.$add mul=calc.$mul cpt=calc.$cpt
print('Result:', '\n', '$add:', add, '\n', '$mul:', mul, '\n', '$cpt:', cpt, '\n')
"

bash ../run.sh

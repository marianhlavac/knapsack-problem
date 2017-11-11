import subprocess
import re

process = subprocess.Popen('jupyter nbconvert report-homework2.ipynb --execute --to latex', shell=True, stdout=subprocess.PIPE)
process.wait()

f = open('report-homework2.tex', 'r+')
contents = f.read() 

r = re.compile('begin{Verbatim}.*?end{Verbatim}', re.DOTALL)
replacements = 0

for m in re.finditer(r, contents):
    if (m.group().find('{incolor}In') >= 0):
        contents = contents.replace(m.group(), '')
        replacements += 1
        
print('{} inputs replacements made.'.format(replacements))

for rplc in ['\\usepackage[T1]{fontenc}', '\maketitle']:
    contents = contents.replace(rplc, '')
    
contents = contents.replace('\\subsection', '\\section', 1)

print('TeX modifications replacement done.')

f.seek(0)
f.write(contents)
f.truncate()
f.close()

print('File overwritten.')

process2 = subprocess.Popen('xelatex report-homework2.tex', shell=True, stdout=subprocess.PIPE)
process2.wait()

print('PDF generated. Done.')
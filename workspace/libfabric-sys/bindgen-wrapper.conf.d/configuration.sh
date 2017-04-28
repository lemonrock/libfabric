# This file is part of libfabric. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT. No part of libfabric, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2016 The developers of libfabric. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libfabric/master/COPYRIGHT.


bindingsName='libfabric'
rootIncludeFileName='libfabric.h'
macosXHomebrewPackageNames='clang-format'
alpineLinuxPackageNames='rsync make gcc linux-headers libunwind-dev linux-grsec-dev'
clangAdditionalArguments=''
headersFolderPath="$homeFolder"/compile-libfabric.conf.d/temporary/usr/include
libFolderPath="$homeFolder"/compile-libfabric.conf.d/temporary/usr/lib
link='libfabric'
link_kind='static-nobundle'

final_chance_to_tweak()
{
	_final_chance_to_tweak_createRustSyntaxForStaticInlineFunctions()
	{	
		set +f
			clang-format -style="{BasedOnStyle: llvm, IndentWidth: 4, ColumnLimit: 10000, BreakBeforeBraces: Allman, AllowShortFunctionsOnASingleLine: false}" "$headersFolderPath"/rdma/*.h | grep '^static ' >"$temporaryFolderPath"/static-functions.h
		set -f
	
		sed \
			-e 's/^static //g' \
			-e 's/^inline //g' \
			-e '/^FI_DEPRECATED_FUNC /d' \
			-e 's/$/;/g' \
			"$temporaryFolderPath"/static-functions.h \
			>"$temporaryFolderPath"/static-functions-without-noise.h
	
		{
			cat <<-'EOF'
				#include <rdma/fabric.h>
				#include <stdint.h>
			EOF
		
			cat "$temporaryFolderPath"/static-functions-without-noise.h
		}>"$temporaryFolderPath"/bindgen.h
	
		bindgen_wrapper_bindgen "$temporaryFolderPath"/bindgen.h "$temporaryFolderPath"/bindgen.rs 1>/dev/null
	
		bindgen_wrapper_cleanedGeneration "$temporaryFolderPath"/bindgen.rs "$temporaryFolderPath"/bindgen.cleaned.rs
	
		sed \
			-e 's/    pub fn/\tpub fn/g' \
			-e 's/  */ /g' \
			"$temporaryFolderPath"/bindgen.cleaned.rs \
		>"$temporaryFolderPath"/bindgen.more-cleaned.rs
	
		cat "$temporaryFolderPath"/bindgen.more-cleaned.rs \
		| tr '\n' '\r' \
		| sed \
			-e 's/,\r /, /g' \
			-e 's/)\r ->/) ->/g' \
		| tr '\r' '\n' \
		| grep '\tpub fn' \
		| sort \
		| sed \
			-e 's/^\tpub fn //g' \
			-e 's/;$//g' \
			-e 's/(/,/g' \
			-e 's/)/,/g' \
			-e 's/, /,/g' \
		>"$temporaryFolderPath"/bindgen.fn.rs
	}
	
	_final_chance_to_tweak_createEasierToParseCLikeSyntaxForStaticInlineFunctions()
	{
		sed \
			-e 's/;$//g' \
			-e 's/(void)/()/g' \
			-e 's/()//g' \
			-e 's/, /,/g' \
			-e 's/(/,/g' \
			-e 's/\*/* /g' \
			-e 's/)$//g' \
			"$temporaryFolderPath"/static-functions-without-noise.h \
			>"$temporaryFolderPath"/static-functions-without-noise.easier-to-parse
	}
	
	local asClause
	local functionOrArgumentName
	local rustPiece
	_final_chance_to_tweak_asClauseAndName()
	{
		local fieldIndexOneBased="$1"

		local typeAndName="$(printf '%s' "$line" | awk -F',' '{print $'$fieldIndexOneBased'}')"
		
		functionOrArgumentName="$(printf '%s' "$typeAndName" | awk -F' ' '{print $NF}')"
		
		local potentialAsClause="$(printf '%s' "$typeAndName" | awk -F' ' '{printf "%s", $1; for(i=2;i<NF;i++){printf " %s", $i}}')"
		if [ "$potentialAsClause" = 'void' ]; then
			asClause=''
		else
			asClause="$potentialAsClause"
		fi
		
		rustPiece="$(printf '%s' "$rustFunctionLine" | awk -F',' '{print $'$fieldIndexOneBased'}')"
	}
	
	_final_chance_to_tweak_createRustSyntaxForStaticInlineFunctions
	_final_chance_to_tweak_createEasierToParseCLikeSyntaxForStaticInlineFunctions
	
	{
		cat <<EOF

c!
{
	#include "rust_types.h"
	#include <${rootIncludeFileName}>
	#include <stdlib.h>
}
EOF
		
		local line
		while IFS='' read -r line
		do
			_final_chance_to_tweak_asClauseAndName 1
			local returnAsClause="$asClause"
			local functionName="$functionOrArgumentName"
		
			local rustFunctionLine="$(grep '^'"$functionName"',' "$temporaryFolderPath"/bindgen.fn.rs)"
			if [ -z "$rustFunctionLine" ]; then
				printf 'WARN: Could not find function %s in rust bindgen output\n' "$functionName" 1>&2
				continue
			else
				printf 'NOTE: Processing function %s\n' "$functionName" 1>&2
			fi
		
			local argumentsToUseInCCode=''
		
			{
				printf '\nc!\n{\n'
			
				printf '\t#[inline(always)]\n'
				printf '\tfn rust_%s(' "$functionName"
		
				local argumentIndex=2
				while true
				do
					_final_chance_to_tweak_asClauseAndName $argumentIndex
					local argumentAsClause="$asClause"
					local argumentName="$functionOrArgumentName"
					if [ -z "$argumentAsClause" ]; then
						break
					fi
				
					if [ -z "$argumentsToUseInCCode" ]; then
						argumentsToUseInCCode="${argumentName}"
					else
						argumentsToUseInCCode="${argumentsToUseInCCode}, ${argumentName}"
					fi
			
					if [ $argumentIndex -ne 2 ]; then
						printf ', '
					fi
					printf '%s as "%s"' "$rustPiece" "$argumentAsClause"
			
					argumentIndex=$((argumentIndex + 1))
				done
			
				if [ -n "$returnAsClause" ]; then
					local rustPieceForReturn="$rustPiece"
					printf ') %s as "%s"\n' "$rustPieceForReturn" "$returnAsClause"
				else
					printf ')\n'
				fi
			
				printf '\t{\n'
			
				if [ -n "$returnAsClause" ]; then
					printf '\t\treturn %s(%s);\n' "$functionName" "$argumentsToUseInCCode"
				else
					printf '\t\%s(%s);\n' "$functionName" "$argumentsToUseInCCode"
				fi
			
				printf '\t}\n'
				printf '}\n'
			}

		done <"$temporaryFolderPath"/static-functions-without-noise.easier-to-parse
	} >"$temporaryFolderPath"/static-functions.rs
	
	cat "$temporaryFolderPath"/static-functions.rs >>"$outputFolderPath"/lib.rs
}
